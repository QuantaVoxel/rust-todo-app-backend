use crate::modules::user::router::user_routes;
use salvo::{Router, handler};

#[handler]
async fn health_check() -> &'static str {
    "Hello World"
}

pub fn routers() -> Router {
    Router::new().get(health_check).push(user_routes())
}
