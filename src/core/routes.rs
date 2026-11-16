use salvo::prelude::*;

use crate::core::middleware::cors_middleware;
use crate::modules::{
    activity_logs, auth, classifications, connections, lookups, notifications, tasks, users,
};

#[handler]
async fn health_check() -> &'static str {
    "OK"
}

pub fn routes() -> Router {
    let api_v1 = Router::with_path("api/v1")
        .hoop(cors_middleware())
        .push(Router::with_path("health").get(health_check))
        .push(auth::routes())
        .push(users::routes())
        .push(tasks::routes())
        .push(lookups::routes())
        .push(classifications::routes())
        .push(connections::routes())
        .push(notifications::routes())
        .push(activity_logs::routes());

    Router::new().push(api_v1)
}
