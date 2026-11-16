use salvo::prelude::*;
use salvo::serve_static::StaticDir;

use crate::core::middleware::{
    auth_rate_limiter, cors_middleware, global_rate_limiter, security_headers_middleware,
};
use crate::modules::{
    activity_logs, auth, classifications, connections, lookups, notifications, tasks, users,
};

#[handler]
async fn health_check() -> &'static str {
    "OK"
}

pub fn routes() -> Router {
    let auth_router = auth::routes().hoop(auth_rate_limiter());

    let api_v1 = Router::with_path("api/v1")
        .hoop(cors_middleware())
        .hoop(security_headers_middleware)
        .hoop(global_rate_limiter())
        .push(Router::with_path("health").get(health_check))
        .push(auth_router)
        .push(users::routes())
        .push(tasks::routes())
        .push(lookups::routes())
        .push(classifications::routes())
        .push(connections::routes())
        .push(notifications::routes())
        .push(activity_logs::routes())
        .push(
            Router::with_path("uploads/{*path}")
                .get(StaticDir::new(["uploads"]).auto_list(false)),
        );

    Router::new()
        .push(api_v1)
        .push(
            Router::with_path("uploads/{*path}")
                .get(StaticDir::new(["uploads"]).auto_list(false)),
        )
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

    #[tokio::test]
    async fn test_classifications_require_auth() {
        let router = routes();
        let service = Service::new(router);

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/tags")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/categories")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/labels")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));
    }

    #[tokio::test]
    async fn test_create_tag_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let settings = crate::config::Settings::load();
        let token = crate::utils::jwt::generate_access_token(1, "testuser", &settings.jwt_secret, 3600).unwrap();

        let payload = serde_json::json!({
            "name": ""
        });

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/tags")
            .add_header("Authorization", format!("Bearer {}", token), true)
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_create_subtask_validation_error() {
        let router = routes();
        let service = Service::new(router);

        let settings = crate::config::Settings::load();
        let token = crate::utils::jwt::generate_access_token(1, "testuser", &settings.jwt_secret, 3600).unwrap();

        // Subtask missing title and status_id
        let payload = serde_json::json!({});

        let mut resp = TestClient::post("http://127.0.0.1:8698/api/v1/tasks/1/subtasks")
            .add_header("Authorization", format!("Bearer {}", token), true)
            .json(&payload)
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::BAD_REQUEST));
        let body = resp.take_string().await.unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[tokio::test]
    async fn test_connections_and_notifications_require_auth() {
        let router = routes();
        let service = Service::new(router);

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/connections")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/notifications")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/activity-logs")
            .send(&service)
            .await;
        assert_eq!(resp.status_code, Some(StatusCode::UNAUTHORIZED));
    }

    #[tokio::test]
    async fn test_security_headers_and_cors() {
        let router = routes();
        let service = Service::new(router);

        let resp = TestClient::get("http://127.0.0.1:8698/api/v1/health")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::OK));
        let headers = resp.headers();
        assert_eq!(
            headers.get("x-content-type-options").and_then(|v| v.to_str().ok()),
            Some("nosniff")
        );
        assert_eq!(
            headers.get("x-frame-options").and_then(|v| v.to_str().ok()),
            Some("DENY")
        );
        assert_eq!(
            headers.get("referrer-policy").and_then(|v| v.to_str().ok()),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            headers.get("access-control-allow-origin").and_then(|v| v.to_str().ok()),
            Some("*")
        );
    }

    #[tokio::test]
    async fn test_static_file_serving() {
        tokio::fs::create_dir_all("uploads").await.unwrap();
        let test_file_path = "uploads/test_asset.txt";
        tokio::fs::write(test_file_path, "Hello from Todo App static upload!").await.unwrap();

        let router = routes();
        let service = Service::new(router);

        let mut resp = TestClient::get("http://127.0.0.1:8698/uploads/test_asset.txt")
            .send(&service)
            .await;

        assert_eq!(resp.status_code, Some(StatusCode::OK));
        let body = resp.take_string().await.unwrap();
        assert_eq!(body, "Hello from Todo App static upload!");

        // Also test through /api/v1/uploads/test_asset.txt
        let mut resp_v1 = TestClient::get("http://127.0.0.1:8698/api/v1/uploads/test_asset.txt")
            .send(&service)
            .await;
        assert_eq!(resp_v1.status_code, Some(StatusCode::OK));
        let body_v1 = resp_v1.take_string().await.unwrap();
        assert_eq!(body_v1, "Hello from Todo App static upload!");

        let _ = tokio::fs::remove_file(test_file_path).await;
    }
}
