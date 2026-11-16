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

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let router = routes();
        let service = Service::new(router);

        let mut resp = TestClient::get("http://127.0.0.1:8698/api/v1/health")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::OK));
        let body = resp.take_string().await.unwrap();
        assert_eq!(body, "OK");
    }

    #[tokio::test]
    async fn test_protected_tasks_requires_authorization() {
        let router = routes();
        let service = Service::new(router);

        let mut resp = TestClient::get("http://127.0.0.1:8698/api/v1/tasks")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("UNAUTHORIZED"));
    }

    #[tokio::test]
    async fn test_reference_enums_accessible_publicly() {
        let router = routes();
        let service = Service::new(router);

        let mut resp = TestClient::get("http://127.0.0.1:8698/api/v1/enums")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::OK));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("vote_type"));
    }

    #[tokio::test]
    async fn test_method_not_allowed_or_error_catcher() {
        let router = routes();
        let service = Service::new(router).catcher(Catcher::default().hoop(crate::core::middleware::error_catcher));

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/health")
            .send(&service)
            .await;

        // Either METHOD_NOT_ALLOWED (405) or NOT_FOUND (404)
        let status = resp.status_code.unwrap();
        assert!(status == StatusCode::METHOD_NOT_ALLOWED || status == StatusCode::NOT_FOUND);

        let body = resp.take_string().await.unwrap();
        assert!(body.contains(r#"{"error":{"code":"#));
    }

    #[tokio::test]
    async fn test_register_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let payload = serde_json::json!({
            "username": "ab", // too short, min 3
            "email": "not-an-email",
            "password": "123" // too short, min 8
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/register")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_login_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let payload = serde_json::json!({
            "email": "invalid-email"
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/login")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_change_password_requires_auth() {
        let router = routes();
        let service = Service::new(router);

        let payload = serde_json::json!({
            "old_password": "password123",
            "new_password": "newpassword123"
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/change-password")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("UNAUTHORIZED"));
    }
}
