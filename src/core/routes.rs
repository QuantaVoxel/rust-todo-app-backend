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
    async fn test_register_missing_profile_fields_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let payload = serde_json::json!({
            "username": "validuser",
            "email": "validuser@example.com",
            "password": "validpassword123"
            // first_name, last_name, phone_number omitted
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/register")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
        assert!(body.contains("first_name") || body.contains("last_name") || body.contains("phone_number"));
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

    #[tokio::test]
    async fn test_refresh_token_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let payload = serde_json::json!({});

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/refresh")
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_logout_endpoint() {
        let router = routes();
        let service = Service::new(router);

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/auth/logout")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::OK));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("Logout berhasil"));
    }

    #[tokio::test]
    async fn test_users_endpoints_require_authorization() {
        let router = routes();
        let service = Service::new(router);

        let endpoints = vec![
            "http://127.0.0.1:8698/api/v1/users",
            "http://127.0.0.1:8698/api/v1/users/me",
            "http://127.0.0.1:8698/api/v1/users/me/profile",
            "http://127.0.0.1:8698/api/v1/users/me/settings",
        ];

        for ep in endpoints {
            let mut resp = TestClient::get(ep).send(&service).await;
            assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));
            let body = resp.take_string().await.unwrap();
            assert!(body.contains("UNAUTHORIZED"));
        }
    }

    #[tokio::test]
    async fn test_update_me_validation_error() {
        let router = routes();
        let service = Service::new(router);

        // Generate a valid access token for user 1
        let settings = crate::config::Settings::load();
        let token = crate::utils::jwt::generate_access_token(1, "testuser", &settings.jwt_secret, 3600).unwrap();

        let payload = serde_json::json!({
            "email": "invalid-email-format"
        });

        let mut resp = TestClient::patch("http://127.0.0.1:8698/api/v1/users/me")
            .add_header("Authorization", format!("Bearer {}", token), true)
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_create_task_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let settings = crate::config::Settings::load();
        let token = crate::utils::jwt::generate_access_token(1, "testuser", &settings.jwt_secret, 3600).unwrap();

        // Missing required title and status_id
        let payload = serde_json::json!({
            "description": "Some description"
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/tasks")
            .add_header("Authorization", format!("Bearer {}", token), true)
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_update_progress_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let settings = crate::config::Settings::load();
        let token = crate::utils::jwt::generate_access_token(1, "testuser", &settings.jwt_secret, 3600).unwrap();

        // Progress percentage must be between 0 and 100
        let payload = serde_json::json!({
            "progress_percentage": 150
        });

        let mut resp = TestClient::put("http://127.0.0.1:8698/api/v1/tasks/1/progress")
            .add_header("Authorization", format!("Bearer {}", token), true)
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }
}
