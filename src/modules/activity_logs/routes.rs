use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::activity_logs::handlers;

pub fn routes() -> Router {
    Router::with_path("activity-logs")
        .hoop(auth_middleware)
        .get(handlers::list_activity_logs)
}
