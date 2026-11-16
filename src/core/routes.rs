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

#[cfg(test)]
mod tests {
    use super::*;
    use salvo::catcher::Catcher;
    use salvo::test::{ResponseExt, TestClient};
    use crate::core::middleware::not_found_handler;

    #[tokio::test]
    async fn test_route_not_found_returns_json_and_404() {
        let router = routes();
        let service = Service::new(router).catcher(Catcher::default().hoop(not_found_handler));

        let mut resp = TestClient::get("http://127.0.0.1:8698/api/v1/nonexistent-route")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::NOT_FOUND));

        let body = resp.take_string().await.unwrap();
        assert_eq!(
            body,
            r#"{"error":{"code":"NOT_FOUND","message":"Route tidak ditemukan"}}"#
        );
    }
}
