use salvo::prelude::*;

use crate::config::Settings;
use crate::error::ApiError;
use crate::utils::jwt::verify_access_token;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub username: String,
    pub token_jti: String,
}

#[handler]
pub async fn auth_middleware(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let auth_header = match req.header::<String>("Authorization") {
        Some(h) => h,
        None => {
            res.render(ApiError::Unauthorized("Authorization header missing".to_string()));
            return;
        }
    };

    let token = if let Some(stripped) = auth_header.strip_prefix("Bearer ") {
        stripped.trim()
    } else {
        res.render(ApiError::Unauthorized("Invalid token format, expected 'Bearer <token>'".to_string()));
        return;
    };

    let settings = Settings::load();
    match verify_access_token(token, &settings.jwt_secret) {
        Ok(claims) => {
            depot.insert_typed(AuthenticatedUser {
                user_id: claims.sub,
                username: claims.username,
                token_jti: claims.jti,
            });
        }
        Err(err) => {
            res.render(err);
        }
    }
}

/// Helper to get authenticated user from Depot in protected handlers
pub fn get_authenticated_user(depot: &Depot) -> Result<&AuthenticatedUser, ApiError> {
    depot
        .get_typed::<AuthenticatedUser>()
        .map_err(|_| ApiError::Unauthorized("Sesi autentikasi tidak ditemukan".to_string()))
}
