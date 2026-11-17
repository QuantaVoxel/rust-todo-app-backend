use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use sea_orm::entity::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::entity::sea_orm_active_enums::NotificationTypeEnum;
use crate::entity::user_notifications;

const BROADCAST_CAPACITY: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeNotificationEvent {
    pub event: String,
    pub notification_id: i32,
    pub user_id: i32,
    pub notification_type: NotificationTypeEnum,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTimeWithTimeZone,
}

impl From<&user_notifications::Model> for RealtimeNotificationEvent {
    fn from(model: &user_notifications::Model) -> Self {
        Self {
            event: "notification.created".to_string(),
            notification_id: model.notification_id,
            user_id: model.user_id,
            notification_type: model.notification_type.clone(),
            message: model.message.clone(),
            is_read: model.is_read,
            created_at: model.created_at,
        }
    }
}

pub struct NotificationHub {
    tx: broadcast::Sender<RealtimeNotificationEvent>,
    active_connections: AtomicUsize,
}

impl NotificationHub {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            active_connections: AtomicUsize::new(0),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RealtimeNotificationEvent> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: RealtimeNotificationEvent) -> usize {
        self.tx.send(event).unwrap_or(0)
    }

    pub fn publish_model(&self, model: &user_notifications::Model) -> usize {
        self.publish(RealtimeNotificationEvent::from(model))
    }

    pub fn active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    pub fn track_connection(&'static self) -> ConnectionGuard {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        ConnectionGuard { hub: self }
    }
}

pub struct ConnectionGuard {
    hub: &'static NotificationHub,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.hub.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
}

static GLOBAL_NOTIFICATION_HUB: LazyLock<NotificationHub> =
    LazyLock::new(|| NotificationHub::new(BROADCAST_CAPACITY));

pub fn notification_hub() -> &'static NotificationHub {
    &GLOBAL_NOTIFICATION_HUB
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_notification_hub_publish_and_subscribe() {
        let hub = NotificationHub::new(16);
        let mut rx1 = hub.subscribe();
        let mut rx2 = hub.subscribe();

        let model = user_notifications::Model {
            notification_id: 42,
            user_id: 7,
            message: "Task dibagikan kepada Anda".to_string(),
            notification_type: NotificationTypeEnum::Share,
            is_read: false,
            created_at: Utc::now().into(),
        };

        let delivered = hub.publish_model(&model);
        assert_eq!(delivered, 2);

        let evt1 = rx1.recv().await.expect("rx1 should receive event");
        let evt2 = rx2.recv().await.expect("rx2 should receive event");

        assert_eq!(evt1.notification_id, 42);
        assert_eq!(evt1.user_id, 7);
        assert_eq!(evt1.event, "notification.created");
        assert_eq!(evt1.notification_type, NotificationTypeEnum::Share);
        assert_eq!(evt1, evt2);
    }

    #[test]
    fn test_connection_guard_tracks_active_connections() {
        let hub = notification_hub();
        let before = hub.active_connections();
        {
            let _g1 = hub.track_connection();
            assert_eq!(hub.active_connections(), before + 1);
            {
                let _g2 = hub.track_connection();
                assert_eq!(hub.active_connections(), before + 2);
            }
            assert_eq!(hub.active_connections(), before + 1);
        }
        assert_eq!(hub.active_connections(), before);
    }
}
