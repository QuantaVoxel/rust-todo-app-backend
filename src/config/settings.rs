use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,      // seconds (e.g. 3600 = 1 hour)
    pub jwt_refresh_expiration: i64, // seconds (e.g. 2592000 = 30 days)
}

impl Settings {
    pub fn load() -> Self {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = env::var("APP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:@localhost:5432/todo_app".to_string());
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_jwt_secret_key_change_in_production_123456789".to_string());
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .ok()
            .and_then(|e| e.parse().ok())
            .unwrap_or(3600); // 1 hour
        let jwt_refresh_expiration = env::var("JWT_REFRESH_EXPIRATION")
            .ok()
            .and_then(|e| e.parse().ok())
            .unwrap_or(3600 * 24 * 30); // 30 days

        Self {
            app_host,
            app_port,
            database_url,
            jwt_secret,
            jwt_expiration,
            jwt_refresh_expiration,
        }
    }
}
