use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::notifications::handlers;

pub fn routes() -> Router {
    Router::with_path("notifications")
        .hoop(auth_middleware)
        .get(handlers::list_notifications)
        .push(Router::with_path("unread-count").get(handlers::get_unread_count))
        .push(Router::with_path("read-all").post(handlers::mark_all_read))
        .push(
            Router::with_path("{notification_id}")
                .patch(handlers::mark_notification_read)
                .delete(handlers::delete_notification),
        )
}
