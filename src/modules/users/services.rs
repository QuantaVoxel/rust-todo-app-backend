use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect, Set};

use crate::core::db::connection::try_get_db_connection;
use crate::entity::{profiles, user_settings, users};
use crate::error::api_error::ApiError;
use crate::modules::users::dtos::*;
use crate::utils::jwt::revoke_token;

pub struct UserService;

impl UserService {
    pub async fn get_me(user_id: i32) -> Result<UserMeResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("User tidak ditemukan".into()))?;

        let profile = Self::get_profile(user_id).await.ok();

        Ok(UserMeResponse {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
            profile,
        })
    }

    pub async fn update_me(
        user_id: i32,
        dto: UpdateUserRequest,
    ) -> Result<UserMeResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let user = users::Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("User tidak ditemukan".into()))?;

        let mut user_active: users::ActiveModel = user.into();
        let mut modified = false;

        if let Some(ref new_username) = dto.username {
            let conflict = users::Entity::find()
                .filter(
                    Condition::all()
                        .add(users::Column::UserId.ne(user_id))
                        .add(users::Column::Username.eq(new_username)),
                )
                .one(db)
                .await?;

            if conflict.is_some() {
                return Err(ApiError::Conflict("Username sudah digunakan".into()));
            }

            user_active.username = Set(new_username.clone());
            modified = true;
        }

        if let Some(ref new_email) = dto.email {
            let conflict = users::Entity::find()
                .filter(
                    Condition::all()
                        .add(users::Column::UserId.ne(user_id))
                        .add(users::Column::Email.eq(new_email)),
                )
                .one(db)
                .await?;

            if conflict.is_some() {
                return Err(ApiError::Conflict("Email sudah digunakan".into()));
            }

            user_active.email = Set(new_email.clone());
            modified = true;
        }

        if modified {
            user_active.updated_at = Set(Utc::now().into());
            user_active.update(db).await?;
        }

        Self::get_me(user_id).await
    }

    pub async fn delete_me(user_id: i32, token_jti: Option<&str>) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        if let Some(jti) = token_jti {
            revoke_token(jti);
        }

        let res = users::Entity::delete_by_id(user_id).exec(db).await?;
        if res.rows_affected == 0 {
            return Err(ApiError::NotFound("User tidak ditemukan".into()));
        }

        Ok(())
    }

    pub async fn search_users(
        current_user_id: i32,
        search: &str,
    ) -> Result<Vec<UserSearchResult>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let mut query = users::Entity::find()
            .filter(users::Column::UserId.ne(current_user_id));

        let trimmed = search.trim();
        if !trimmed.is_empty() {
            query = query.filter(
                Condition::any()
                    .add(users::Column::Username.contains(trimmed))
                    .add(users::Column::Email.contains(trimmed)),
            );
        }

        let user_list = query.limit(50).all(db).await?;

        Ok(user_list
            .into_iter()
            .map(|u| UserSearchResult {
                user_id: u.user_id,
                username: u.username,
                email: u.email,
            })
            .collect())
    }

    pub async fn get_profile(user_id: i32) -> Result<ProfileResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let profile = profiles::Entity::find()
            .filter(profiles::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        let profile = match profile {
            Some(p) => p,
            None => {
                // Auto-create default profile for user if missing
                let new_profile = profiles::ActiveModel {
                    user_id: Set(user_id),
                    first_name: Set(None),
                    last_name: Set(None),
                    avatar_url: Set(None),
                    phone_number: Set(None),
                    ..Default::default()
                };
                new_profile.insert(db).await?
            }
        };

        Ok(ProfileResponse {
            profile_id: profile.profile_id,
            user_id: profile.user_id,
            first_name: profile.first_name,
            last_name: profile.last_name,
            avatar_url: profile.avatar_url,
            phone_number: profile.phone_number,
        })
    }

    pub async fn upsert_profile(
        user_id: i32,
        dto: UpsertProfileRequest,
    ) -> Result<ProfileResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let existing = profiles::Entity::find()
            .filter(profiles::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        let saved = match existing {
            Some(p) => {
                let mut active: profiles::ActiveModel = p.into();
                if dto.first_name.is_some() {
                    active.first_name = Set(dto.first_name);
                }
                if dto.last_name.is_some() {
                    active.last_name = Set(dto.last_name);
                }
                if dto.avatar_url.is_some() {
                    active.avatar_url = Set(dto.avatar_url);
                }
                if dto.phone_number.is_some() {
                    active.phone_number = Set(dto.phone_number);
                }
                active.update(db).await?
            }
            None => {
                let new_profile = profiles::ActiveModel {
                    user_id: Set(user_id),
                    first_name: Set(dto.first_name),
                    last_name: Set(dto.last_name),
                    avatar_url: Set(dto.avatar_url),
                    phone_number: Set(dto.phone_number),
                    ..Default::default()
                };
                new_profile.insert(db).await?
            }
        };

        Ok(ProfileResponse {
            profile_id: saved.profile_id,
            user_id: saved.user_id,
            first_name: saved.first_name,
            last_name: saved.last_name,
            avatar_url: saved.avatar_url,
            phone_number: saved.phone_number,
        })
    }

    pub async fn get_settings(user_id: i32) -> Result<Vec<UserSettingResponse>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let list = user_settings::Entity::find()
            .filter(user_settings::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(list
            .into_iter()
            .map(|s| UserSettingResponse {
                setting_id: s.setting_id,
                user_id: s.user_id,
                setting_key: s.setting_key,
                setting_value: s.setting_value,
            })
            .collect())
    }

    pub async fn upsert_setting(
        user_id: i32,
        key: String,
        dto: UpsertSettingRequest,
    ) -> Result<UserSettingResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let existing = user_settings::Entity::find()
            .filter(
                Condition::all()
                    .add(user_settings::Column::UserId.eq(user_id))
                    .add(user_settings::Column::SettingKey.eq(&key)),
            )
            .one(db)
            .await?;

        let saved = match existing {
            Some(s) => {
                let mut active: user_settings::ActiveModel = s.into();
                active.setting_value = Set(dto.setting_value);
                active.update(db).await?
            }
            None => {
                let new_setting = user_settings::ActiveModel {
                    user_id: Set(user_id),
                    setting_key: Set(key),
                    setting_value: Set(dto.setting_value),
                    ..Default::default()
                };
                new_setting.insert(db).await?
            }
        };

        Ok(UserSettingResponse {
            setting_id: saved.setting_id,
            user_id: saved.user_id,
            setting_key: saved.setting_key,
            setting_value: saved.setting_value,
        })
    }

    pub async fn delete_setting(user_id: i32, key: &str) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let existing = user_settings::Entity::find()
            .filter(
                Condition::all()
                    .add(user_settings::Column::UserId.eq(user_id))
                    .add(user_settings::Column::SettingKey.eq(key)),
            )
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Pengaturan '{}' tidak ditemukan", key)))?;

        user_settings::Entity::delete_by_id(existing.setting_id)
            .exec(db)
            .await?;

        Ok(())
    }
}
