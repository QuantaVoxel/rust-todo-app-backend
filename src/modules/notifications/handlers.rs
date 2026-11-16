use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::notifications::dtos::*;

#[handler]
pub async fn list_notifications(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_unread_count(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new(serde_json::json!({ "unread_count": 0 }))));
}

#[handler]
pub async fn mark_notification_read(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _id = req.param::<i32>("notification_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateNotificationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Notification updated")));
}

#[handler]
pub async fn mark_all_read(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("All notifications marked as read")));
}

#[handler]
pub async fn delete_notification(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _id = req.param::<i32>("notification_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Notification deleted")));
}
