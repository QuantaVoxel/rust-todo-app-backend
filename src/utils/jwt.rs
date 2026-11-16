use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i32,          // user_id
    pub username: String,
    pub exp: i64,          // expiration timestamp
    pub iat: i64,          // issued at timestamp
}

pub fn generate_token(
    user_id: i32,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, ApiError> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id,
        username: username.to_string(),
        exp: now + expires_in_seconds,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(ApiError::from)
}

pub fn verify_token(token: &str, secret: &str) -> Result<JwtClaims, ApiError> {
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(ApiError::from)?;

    Ok(token_data.claims)
}
