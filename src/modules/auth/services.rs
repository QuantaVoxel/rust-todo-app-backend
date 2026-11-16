use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};

use crate::config::Settings;
use crate::core::db::connection::try_get_db_connection;
use crate::entity::users;
use crate::error::api_error::ApiError;
use crate::modules::auth::dtos::*;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::jwt::{
    generate_access_token, generate_refresh_token, revoke_token, verify_refresh_token,
};

pub struct AuthService;

impl AuthService {
    pub async fn register(dto: RegisterRequest) -> Result<RegisterResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let username = dto
            .username
            .ok_or_else(|| ApiError::BadRequest("Username wajib diisi".into()))?;
        let email = dto
            .email
            .ok_or_else(|| ApiError::BadRequest("Email wajib diisi".into()))?;
        let password = dto
            .password
            .ok_or_else(|| ApiError::BadRequest("Password wajib diisi".into()))?;
        let first_name = dto
            .first_name
            .ok_or_else(|| ApiError::BadRequest("First name wajib diisi".into()))?;
        let last_name = dto
            .last_name
            .ok_or_else(|| ApiError::BadRequest("Last name wajib diisi".into()))?;
        let phone_number = dto
            .phone_number
            .ok_or_else(|| ApiError::BadRequest("Phone number wajib diisi".into()))?;

        // Check if username or email is already in use
        let existing = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.eq(&username))
                    .add(users::Column::Email.eq(&email)),
            )
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(ApiError::Conflict(
                "Email atau username sudah digunakan".into(),
            ));
        }

        let password_hash = hash_password(&password)?;
        let now = Utc::now().into();

        let new_user = users::ActiveModel {
            username: Set(username),
            email: Set(email),
            password_hash: Set(password_hash),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let user = new_user.insert(db).await?;

        // Save profile record
        let new_profile = crate::entity::profiles::ActiveModel {
            user_id: Set(user.user_id),
            first_name: Set(Some(first_name.clone())),
            last_name: Set(Some(last_name.clone())),
            avatar_url: Set(dto.avatar_url),
            phone_number: Set(Some(phone_number.clone())),
            ..Default::default()
        };
        new_profile.insert(db).await?;

        Ok(RegisterResponse {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            first_name,
            last_name,
            phone_number,
            created_at: user.created_at.with_timezone(&Utc),
        })
    }

    pub async fn login(dto: LoginRequest) -> Result<LoginResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let email = dto
            .email
            .ok_or_else(|| ApiError::BadRequest("Email wajib diisi".into()))?;
        let password = dto
            .password
            .ok_or_else(|| ApiError::BadRequest("Password wajib diisi".into()))?;

        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&email))
            .one(db)
            .await?
            .ok_or_else(|| ApiError::Unauthorized("Email atau kata sandi salah".into()))?;

        let is_valid = verify_password(&password, &user.password_hash)?;
        if !is_valid {
            return Err(ApiError::Unauthorized("Email atau kata sandi salah".into()));
        }

        let settings = Settings::load();
        let access_token = generate_access_token(
            user.user_id,
            &user.username,
            &settings.jwt_secret,
            settings.jwt_expiration,
        )?;

        let refresh_token = generate_refresh_token(
            user.user_id,
            &user.username,
            &settings.jwt_secret,
            settings.jwt_refresh_expiration,
        )?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            expires_in: settings.jwt_expiration,
            user: UserSummary {
                user_id: user.user_id,
                username: user.username,
            },
        })
    }

    pub async fn refresh_token(dto: RefreshTokenRequest) -> Result<RefreshTokenResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let token = dto
            .refresh_token
            .ok_or_else(|| ApiError::BadRequest("Refresh token wajib diisi".into()))?;

        let settings = Settings::load();
        let claims = verify_refresh_token(&token, &settings.jwt_secret)?;

        let user = users::Entity::find_by_id(claims.sub)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::Unauthorized("User tidak ditemukan".into()))?;

        // Invalidate old refresh token (Token Rotation)
        revoke_token(&claims.jti);

        let new_access_token = generate_access_token(
            user.user_id,
            &user.username,
            &settings.jwt_secret,
            settings.jwt_expiration,
        )?;

        let new_refresh_token = generate_refresh_token(
            user.user_id,
            &user.username,
            &settings.jwt_secret,
            settings.jwt_refresh_expiration,
        )?;

        Ok(RefreshTokenResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in: settings.jwt_expiration,
            user: UserSummary {
                user_id: user.user_id,
                username: user.username,
            },
        })
    }

    pub async fn logout(
        dto: Option<LogoutRequest>,
        auth_jti: Option<&str>,
    ) -> Result<(), ApiError> {
        let settings = Settings::load();

        // Revoke refresh token if provided
        if let Some(req) = dto {
            if let Some(ref rt) = req.refresh_token {
                if let Ok(claims) = verify_refresh_token(rt, &settings.jwt_secret) {
                    revoke_token(&claims.jti);
                }
            }
        }

        // Revoke active bearer token if user is authenticated
        if let Some(jti) = auth_jti {
            revoke_token(jti);
        }

        Ok(())
    }

    pub async fn change_password(
        user_id: i32,
        dto: ChangePasswordRequest,
    ) -> Result<(), ApiError> {
        let old_pw = dto
            .old_password
            .ok_or_else(|| ApiError::BadRequest("Kata sandi lama wajib diisi".into()))?;
        let new_pw = dto
            .new_password
            .ok_or_else(|| ApiError::BadRequest("Kata sandi baru wajib diisi".into()))?;

        if let Some(ref conf) = dto.new_password_confirmation {
            if conf != &new_pw {
                return Err(ApiError::BadRequest(
                    "Konfirmasi password baru tidak cocok".into(),
                ));
            }
        }

        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("User tidak ditemukan".into()))?;

        let is_valid = verify_password(&old_pw, &user.password_hash)?;
        if !is_valid {
            return Err(ApiError::BadRequest("Kata sandi lama tidak sesuai".into()));
        }

        let new_hash = hash_password(&new_pw)?;
        let mut user_active: users::ActiveModel = user.into();
        user_active.password_hash = Set(new_hash);
        user_active.updated_at = Set(Utc::now().into());

        user_active.update(db).await?;

        Ok(())
    }
}
