use std::convert::Infallible;
use std::time::Duration;
use salvo::oapi::endpoint;
use salvo::prelude::*;
use salvo::sse::{SseEvent, SseKeepAlive};
use salvo::websocket::{Message, WebSocketUpgrade};
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::core::realtime::notification_hub;
use crate::modules::notifications::dtos::*;
use crate::modules::notifications::services::NotificationService;

#[endpoint(tags("Notifications"), summary = "List notifications with optional is_read filter", security(("bearer_auth" = [])))]
pub async fn list_notifications(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let is_read_filter = req.query::<bool>("is_read");

    match NotificationService::list_notifications(user.user_id, is_read_filter).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Get unread notifications count", security(("bearer_auth" = [])))]
pub async fn get_unread_count(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::get_unread_count(user.user_id).await {
        Ok(count) => res.render(Json(ApiResponse::new(serde_json::json!({
            "unread_count": count
        })))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Mark single notification as read or unread", security(("bearer_auth" = [])))]
pub async fn mark_notification_read(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let id = req.param::<i32>("notification_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateNotificationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::mark_notification_read(user.user_id, id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Mark all notifications as read", security(("bearer_auth" = [])))]
pub async fn mark_all_read(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match NotificationService::mark_all_read(user.user_id).await {
        Ok(_) => res.render(Json(ApiResponse::new(
            "Semua notifikasi telah ditandai dibaca",
        ))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Notifications"), summary = "Delete notification by ID", security(("bearer_auth" = [])))]
pub async fn delete_notification(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let id = req.param::<i32>("notification_id").unwrap_or_default();

    match NotificationService::delete_notification(user.user_id, id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Notifikasi berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

#[endpoint(
    tags("Notifications"),
    summary = "Subscribe to real-time notifications via Server-Sent Events (SSE)",
    security(("bearer_auth" = []))
)]
pub async fn sse_notifications(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u.clone(),
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let user_id = user.user_id;
    let hub = notification_hub();
    let mut rx = hub.subscribe();
    let _guard = hub.track_connection();

    let stream = async_stream::stream! {
        let _keep_guard = _guard;
        if let Ok(init_evt) = SseEvent::default()
            .name("connected")
            .json(&serde_json::json!({
                "event": "connected",
                "user_id": user_id,
                "transport": "sse"
            }))
        {
            yield Ok::<_, Infallible>(init_evt);
        }

        loop {
            match rx.recv().await {
                Ok(event) => {
                    if event.user_id == user_id {
                        if let Ok(sse_evt) = SseEvent::default()
                            .name("notification")
                            .id(event.notification_id.to_string())
                            .json(&event)
                        {
                            yield Ok::<_, Infallible>(sse_evt);
                        }
                    }
                }
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => break,
            }
        }
    };

    SseKeepAlive::new(stream)
        .max_interval(Duration::from_secs(15))
        .comment("keep-alive")
        .stream(res);
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WsClientCommand {
    Ping,
    MarkRead { notification_id: i32 },
}

#[endpoint(
    tags("Notifications"),
    summary = "Connect to real-time bidirectional notifications via WebSocket",
    security(("bearer_auth" = []))
)]
pub async fn ws_notifications(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u.clone(),
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let user_id = user.user_id;
    let upgrade_res = WebSocketUpgrade::new()
        .upgrade(req, res, move |mut ws| async move {
            let hub = notification_hub();
            let _guard = hub.track_connection();
            let mut rx = hub.subscribe();
            let mut heartbeat = tokio::time::interval(Duration::from_secs(30));
            // Skip first immediate tick
            heartbeat.tick().await;

            let welcome = serde_json::json!({
                "event": "connected",
                "user_id": user_id,
                "transport": "websocket"
            });
            if let Ok(txt) = serde_json::to_string(&welcome) {
                if ws.send(Message::text(txt)).await.is_err() {
                    return;
                }
            }

            loop {
                tokio::select! {
                    recv_res = rx.recv() => {
                        match recv_res {
                            Ok(event) if event.user_id == user_id => {
                                if let Ok(json_txt) = serde_json::to_string(&event) {
                                    if ws.send(Message::text(json_txt)).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(RecvError::Lagged(_)) => continue,
                            Err(RecvError::Closed) => break,
                        }
                    }
                    ws_msg = ws.recv() => {
                        match ws_msg {
                            Some(Ok(msg)) => {
                                if msg.is_close() {
                                    break;
                                }
                                if msg.is_text() {
                                    if let Ok(text) = msg.as_str() {
                                        if let Ok(cmd) = serde_json::from_str::<WsClientCommand>(text) {
                                            match cmd {
                                                WsClientCommand::Ping => {
                                                    let pong = serde_json::json!({ "event": "pong" });
                                                    if let Ok(p) = serde_json::to_string(&pong) {
                                                        let _ = ws.send(Message::text(p)).await;
                                                    }
                                                }
                                                WsClientCommand::MarkRead { notification_id } => {
                                                    let dto = UpdateNotificationRequest { is_read: Some(true) };
                                                    if let Ok(updated) = NotificationService::mark_notification_read(user_id, notification_id, dto).await {
                                                        let ack = serde_json::json!({
                                                            "event": "notification.updated",
                                                            "notification_id": updated.notification_id,
                                                            "is_read": updated.is_read
                                                        });
                                                        if let Ok(a) = serde_json::to_string(&ack) {
                                                            let _ = ws.send(Message::text(a)).await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Some(Err(_)) | None => break,
                        }
                    }
                    _ = heartbeat.tick() => {
                        if ws.send(Message::ping(Vec::new())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        })
        .await;

    if let Err(err) = upgrade_res {
        res.render(err);
    }
}

