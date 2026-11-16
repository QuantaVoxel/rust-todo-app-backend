use salvo::{Router, handler};

#[handler]
async fn health_check() -> &'static str {
    "Hello World"
}

pub fn routes() -> Router {
    Router::with_path("api/v1").get(health_check)
}
