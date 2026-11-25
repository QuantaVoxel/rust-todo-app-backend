use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::core::cache::app_cache;
use crate::core::db::connection::try_get_db_connection;
use crate::entity::{
    tags, task_categories, task_category_assignment, task_label_assignment, task_labels, task_tags,
};
use crate::error::api_error::ApiError;
use crate::modules::classifications::dtos::*;

pub struct ClassificationService;

impl ClassificationService {
    // ----------------------------------------------------
    // Tags
    // ----------------------------------------------------

    pub async fn list_tags() -> Result<Vec<tags::Model>, ApiError> {
        if let Some(cached) = app_cache().get_tags().await {
            return Ok(cached);
        }
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let items = tags::Entity::find()
            .order_by_asc(tags::Column::TagId)
            .all(db)
            .await?;
        app_cache().set_tags(items.clone()).await;
        Ok(items)
    }

    pub async fn create_tag(dto: TagRequest) -> Result<tags::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name tag wajib diisi".into()))?;

        let existing = tags::Entity::find()
            .filter(tags::Column::Name.eq(&name))
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(ApiError::Conflict("Tag dengan nama tersebut sudah ada".into()));
        }

        let new_tag = tags::ActiveModel {
            name: Set(name),
            ..Default::default()
        };
        let created = new_tag.insert(db).await?;
        app_cache().invalidate_tags().await;
        Ok(created)
    }

    pub async fn update_tag(tag_id: i32, dto: TagRequest) -> Result<tags::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let tag = tags::Entity::find_by_id(tag_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Tag tidak ditemukan".into()))?;

        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name tag wajib diisi".into()))?;

        if tag.name != name {
            let conflict = tags::Entity::find()
                .filter(tags::Column::Name.eq(&name))
                .one(db)
                .await?;
            if conflict.is_some() {
                return Err(ApiError::Conflict("Tag dengan nama tersebut sudah ada".into()));
            }
        }

        let mut active: tags::ActiveModel = tag.into();
        active.name = Set(name);
        let updated = active.update(db).await?;
        app_cache().invalidate_tags().await;
        Ok(updated)
    }

    pub async fn delete_tag(tag_id: i32, force: bool) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let _ = tags::Entity::find_by_id(tag_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Tag tidak ditemukan".into()))?;

        if !force {
            let count = task_tags::Entity::find()
                .filter(task_tags::Column::TagId.eq(tag_id))
                .count(db)
                .await?;
            if count > 0 {
                return Err(ApiError::Conflict(
                    "Tag sedang digunakan pada task. Gunakan query ?force=true untuk menghapus beserta relasinya".into(),
                ));
            }
        }

        tags::Entity::delete_by_id(tag_id).exec(db).await?;
        app_cache().invalidate_tags().await;
        Ok(())
    }

    // ----------------------------------------------------
    // Categories
    // ----------------------------------------------------

    pub async fn list_categories() -> Result<Vec<task_categories::Model>, ApiError> {
        if let Some(cached) = app_cache().get_categories().await {
            return Ok(cached);
        }
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let items = task_categories::Entity::find()
            .order_by_asc(task_categories::Column::CategoryId)
            .all(db)
            .await?;
        app_cache().set_categories(items.clone()).await;
        Ok(items)
    }

    pub async fn create_category(dto: CategoryRequest) -> Result<task_categories::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name category wajib diisi".into()))?;

        let existing = task_categories::Entity::find()
            .filter(task_categories::Column::Name.eq(&name))
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(ApiError::Conflict(
                "Kategori dengan nama tersebut sudah ada".into(),
            ));
        }

        let new_cat = task_categories::ActiveModel {
            name: Set(name),
            description: Set(dto.description),
            ..Default::default()
        };
        let created = new_cat.insert(db).await?;
        app_cache().invalidate_categories().await;
        Ok(created)
    }

    pub async fn update_category(
        category_id: i32,
        dto: CategoryRequest,
    ) -> Result<task_categories::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let cat = task_categories::Entity::find_by_id(category_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Kategori tidak ditemukan".into()))?;

        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name category wajib diisi".into()))?;

        if cat.name != name {
            let conflict = task_categories::Entity::find()
                .filter(task_categories::Column::Name.eq(&name))
                .one(db)
                .await?;
            if conflict.is_some() {
                return Err(ApiError::Conflict(
                    "Kategori dengan nama tersebut sudah ada".into(),
                ));
            }
        }

        let mut active: task_categories::ActiveModel = cat.into();
        active.name = Set(name);
        if dto.description.is_some() {
            active.description = Set(dto.description);
        }
        let updated = active.update(db).await?;
        app_cache().invalidate_categories().await;
        Ok(updated)
    }

    pub async fn delete_category(category_id: i32, force: bool) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let _ = task_categories::Entity::find_by_id(category_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Kategori tidak ditemukan".into()))?;

        if !force {
            let count = task_category_assignment::Entity::find()
                .filter(task_category_assignment::Column::CategoryId.eq(category_id))
                .count(db)
                .await?;
            if count > 0 {
                return Err(ApiError::Conflict(
                    "Kategori sedang digunakan pada task. Gunakan query ?force=true untuk menghapus beserta relasinya".into(),
                ));
            }
        }

        task_categories::Entity::delete_by_id(category_id).exec(db).await?;
        app_cache().invalidate_categories().await;
        Ok(())
    }

    // ----------------------------------------------------
    // Labels
    // ----------------------------------------------------

    pub async fn list_labels() -> Result<Vec<task_labels::Model>, ApiError> {
        if let Some(cached) = app_cache().get_labels().await {
            return Ok(cached);
        }
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let items = task_labels::Entity::find()
            .order_by_asc(task_labels::Column::LabelId)
            .all(db)
            .await?;
        app_cache().set_labels(items.clone()).await;
        Ok(items)
    }

    pub async fn create_label(dto: LabelRequest) -> Result<task_labels::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name label wajib diisi".into()))?;

        let existing = task_labels::Entity::find()
            .filter(task_labels::Column::Name.eq(&name))
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(ApiError::Conflict("Label dengan nama tersebut sudah ada".into()));
        }

        let new_lbl = task_labels::ActiveModel {
            name: Set(name),
            ..Default::default()
        };
        let created = new_lbl.insert(db).await?;
        app_cache().invalidate_labels().await;
        Ok(created)
    }

    pub async fn update_label(
        label_id: i32,
        dto: LabelRequest,
    ) -> Result<task_labels::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let lbl = task_labels::Entity::find_by_id(label_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Label tidak ditemukan".into()))?;

        let name = dto
            .name
            .ok_or_else(|| ApiError::BadRequest("name label wajib diisi".into()))?;

        if lbl.name != name {
            let conflict = task_labels::Entity::find()
                .filter(task_labels::Column::Name.eq(&name))
                .one(db)
                .await?;
            if conflict.is_some() {
                return Err(ApiError::Conflict("Label dengan nama tersebut sudah ada".into()));
            }
        }

        let mut active: task_labels::ActiveModel = lbl.into();
        active.name = Set(name);
        let updated = active.update(db).await?;
        app_cache().invalidate_labels().await;
        Ok(updated)
    }

    pub async fn delete_label(label_id: i32, force: bool) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let _ = task_labels::Entity::find_by_id(label_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Label tidak ditemukan".into()))?;

        if !force {
            let count = task_label_assignment::Entity::find()
                .filter(task_label_assignment::Column::LabelId.eq(label_id))
                .count(db)
                .await?;
            if count > 0 {
                return Err(ApiError::Conflict(
                    "Label sedang digunakan pada task. Gunakan query ?force=true untuk menghapus beserta relasinya".into(),
                ));
            }
        }

        task_labels::Entity::delete_by_id(label_id).exec(db).await?;
        app_cache().invalidate_labels().await;
        Ok(())
    }
}
