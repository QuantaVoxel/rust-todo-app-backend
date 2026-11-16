use std::collections::HashSet;
use std::sync::{LazyLock, RwLock};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;

static REVOKED_TOKENS: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub fn revoke_token(jti: &str) {
    if let Ok(mut set) = REVOKED_TOKENS.write() {
        set.insert(jti.to_string());
    }
}

pub fn is_token_revoked(jti: &str) -> bool {
    if let Ok(set) = REVOKED_TOKENS.read() {
        set.contains(jti)
    } else {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: i32,                 // user_id
    pub username: String,
    pub token_type: String,       // "access" | "refresh"
    pub exp: i64,                 // expiration timestamp
    pub iat: i64,                 // issued at timestamp
    pub jti: String,              // unique token identifier
}

pub fn generate_token(
    user_id: i32,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, ApiError> {
    generate_access_token(user_id, username, secret, expires_in_seconds)
}

pub fn generate_access_token(
    user_id: i32,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, ApiError> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id,
        username: username.to_string(),
        token_type: "access".to_string(),
        exp: now + expires_in_seconds,
        iat: now,
        jti: Uuid::new_v4().to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(ApiError::from)
}

pub fn generate_refresh_token(
    user_id: i32,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, ApiError> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id,
        username: username.to_string(),
        token_type: "refresh".to_string(),
        exp: now + expires_in_seconds,
        iat: now,
        jti: Uuid::new_v4().to_string(),
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

    if is_token_revoked(&token_data.claims.jti) {
        return Err(ApiError::Unauthorized("Token telah dicabut (revoked)".to_string()));
    }

    Ok(token_data.claims)
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<JwtClaims, ApiError> {
    let claims = verify_token(token, secret)?;
    if claims.token_type != "access" {
        return Err(ApiError::Unauthorized("Tipe token bukan access token".to_string()));
    }
    Ok(claims)
}

pub fn verify_refresh_token(token: &str, secret: &str) -> Result<JwtClaims, ApiError> {
    let claims = verify_token(token, secret)?;
    if claims.token_type != "refresh" {
        return Err(ApiError::Unauthorized("Tipe token bukan refresh token".to_string()));
    }
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test_super_secret_jwt_key_1234567890";

    #[test]
    fn test_access_and_refresh_token_type_separation() {
        let access = generate_access_token(1, "user1", SECRET, 3600).unwrap();
        let refresh = generate_refresh_token(1, "user1", SECRET, 7200).unwrap();

        // Access token should verify with verify_access_token
        let access_claims = verify_access_token(&access, SECRET).unwrap();
        assert_eq!(access_claims.token_type, "access");
        assert_eq!(access_claims.sub, 1);

        // Access token should FAIL with verify_refresh_token
        let fail_refresh = verify_refresh_token(&access, SECRET);
        assert!(fail_refresh.is_err());

        // Refresh token should verify with verify_refresh_token
        let refresh_claims = verify_refresh_token(&refresh, SECRET).unwrap();
        assert_eq!(refresh_claims.token_type, "refresh");

        // Refresh token should FAIL with verify_access_token
        let fail_access = verify_access_token(&refresh, SECRET);
        assert!(fail_access.is_err());
    }

    #[test]
    fn test_token_revocation() {
        let refresh = generate_refresh_token(2, "user2", SECRET, 7200).unwrap();
        let claims = verify_refresh_token(&refresh, SECRET).unwrap();

        assert!(!is_token_revoked(&claims.jti));

        // Revoke the token
        revoke_token(&claims.jti);
        assert!(is_token_revoked(&claims.jti));

        // Now verification must fail
        let result = verify_refresh_token(&refresh, SECRET);
        assert!(result.is_err());
    }
}
