use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::common::response::{PageMeta, PaginatedResponse};
use crate::core::db::connection::try_get_db_connection;
use crate::entity::sea_orm_active_enums::SharePermissionEnum;
use crate::entity::{
    activity_logs, task_history, task_priorities, task_priority_history, task_sharing, task_statuses,
    task_tags, tasks,
};
use crate::error::api_error::ApiError;
use crate::modules::tasks::dtos::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskAccessLevel {
    Owner,
    Edit,
    Comment,
    View,
}

pub struct TaskService;

impl TaskService {
    /// Helper to verify task existence and access rights
    pub async fn check_access(
        db: &DatabaseConnection,
        user_id: i32,
        task_id: i32,
    ) -> Result<(tasks::Model, TaskAccessLevel), ApiError> {
        let task = tasks::Entity::find_by_id(task_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Task tidak ditemukan".into()))?;

        if task.user_id == user_id {
            return Ok((task, TaskAccessLevel::Owner));
        }

        let share = task_sharing::Entity::find()
            .filter(
                Condition::all()
                    .add(task_sharing::Column::TaskId.eq(task_id))
                    .add(task_sharing::Column::UserId.eq(user_id)),
            )
            .one(db)
            .await?;

        match share {
            Some(s) => match s.permission {
                SharePermissionEnum::Edit => Ok((task, TaskAccessLevel::Edit)),
                SharePermissionEnum::Comment => Ok((task, TaskAccessLevel::Comment)),
                SharePermissionEnum::View => Ok((task, TaskAccessLevel::View)),
            },
            None => Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengakses task ini".into(),
            )),
        }
    }

    /// Convert tasks::Model to TaskResponse with status and priority names
    pub async fn to_task_response(
        db: &DatabaseConnection,
        task: tasks::Model,
    ) -> TaskResponse {
        let status = task_statuses::Entity::find_by_id(task.status_id)
            .one(db)
            .await
            .ok()
            .flatten()
            .map(|s| s.name);

        let priority = match task.priority_id {
            Some(p_id) => task_priorities::Entity::find_by_id(p_id)
                .one(db)
                .await
                .ok()
                .flatten()
                .map(|p| p.name),
            None => None,
        };

        TaskResponse {
            task_id: task.task_id,
            user_id: task.user_id,
            title: task.title,
            description: task.description,
            status_id: task.status_id,
            status,
            priority_id: task.priority_id,
            priority,
            progress_percentage: task.progress_percentage,
            due_date: task.due_date.map(|d| d.with_timezone(&Utc)),
            archived_at: task.archived_at.map(|d| d.with_timezone(&Utc)),
            created_at: task.created_at.with_timezone(&Utc),
            updated_at: task.updated_at.with_timezone(&Utc),
        }
    }

    pub async fn list_tasks(
        user_id: i32,
        query: TaskListQuery,
    ) -> Result<PaginatedResponse<TaskResponse>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let mut select = tasks::Entity::find();

        // 1. Scope filter (owned, shared, all)
        match query.scope.as_deref() {
            Some("owned") => {
                select = select.filter(tasks::Column::UserId.eq(user_id));
            }
            Some("shared") => {
                let shared_task_ids = task_sharing::Entity::find()
                    .filter(task_sharing::Column::UserId.eq(user_id))
                    .select_only()
                    .column(task_sharing::Column::TaskId)
                    .into_tuple::<i32>()
                    .all(db)
                    .await?;
                select = select.filter(tasks::Column::TaskId.is_in(shared_task_ids));
            }
            _ => {
                // "all" or default: owned OR shared
                let shared_task_ids = task_sharing::Entity::find()
                    .filter(task_sharing::Column::UserId.eq(user_id))
                    .select_only()
                    .column(task_sharing::Column::TaskId)
                    .into_tuple::<i32>()
                    .all(db)
                    .await?;

                select = select.filter(
                    Condition::any()
                        .add(tasks::Column::UserId.eq(user_id))
                        .add(tasks::Column::TaskId.is_in(shared_task_ids)),
                );
            }
        }

        // 2. Archived filter (default: false -> active tasks only)
        if query.archived == Some(true) {
            select = select.filter(tasks::Column::ArchivedAt.is_not_null());
        } else {
            select = select.filter(tasks::Column::ArchivedAt.is_null());
        }

        // 3. Status filter
        if let Some(status_id) = query.status_id {
            select = select.filter(tasks::Column::StatusId.eq(status_id));
        }

        // 4. Priority filter
        if let Some(priority_id) = query.priority_id {
            select = select.filter(tasks::Column::PriorityId.eq(priority_id));
        }

        // 5. Tag filter
        if let Some(tag_id) = query.tag_id {
            let task_ids_with_tag = task_tags::Entity::find()
                .filter(task_tags::Column::TagId.eq(tag_id))
                .select_only()
                .column(task_tags::Column::TaskId)
                .into_tuple::<i32>()
                .all(db)
                .await?;
            select = select.filter(tasks::Column::TaskId.is_in(task_ids_with_tag));
        }

        // 6. Search filter (title / description)
        if let Some(search) = query.search {
            let trimmed = search.trim();
            if !trimmed.is_empty() {
                select = select.filter(
                    Condition::any()
                        .add(tasks::Column::Title.contains(trimmed))
                        .add(tasks::Column::Description.contains(trimmed)),
                );
            }
        }

        // 7. Sort order
        select = match query.sort.as_deref() {
            Some("due_date") => select.order_by(tasks::Column::DueDate, Order::Asc),
            Some("-due_date") => select.order_by(tasks::Column::DueDate, Order::Desc),
            Some("created_at") => select.order_by(tasks::Column::CreatedAt, Order::Asc),
            Some("-created_at") => select.order_by(tasks::Column::CreatedAt, Order::Desc),
            Some("priority") => select.order_by(tasks::Column::PriorityId, Order::Asc),
            Some("-priority") => select.order_by(tasks::Column::PriorityId, Order::Desc),
            _ => select.order_by(tasks::Column::TaskId, Order::Desc),
        };

        // 8. Pagination
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

        let paginator = select.paginate(db, per_page);
        let total = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;
        let task_models = paginator.fetch_page(page - 1).await?;

        let mut data = Vec::with_capacity(task_models.len());
        for task in task_models {
            data.push(Self::to_task_response(db, task).await);
        }

        Ok(PaginatedResponse {
            data,
            meta: PageMeta {
                page,
                per_page,
                total,
                total_pages,
            },
        })
    }

    pub async fn create_task(
        user_id: i32,
        dto: CreateTaskRequest,
    ) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let title = dto
            .title
            .ok_or_else(|| ApiError::BadRequest("Title wajib diisi".into()))?;
        let status_id = dto
            .status_id
            .ok_or_else(|| ApiError::BadRequest("Status ID wajib diisi".into()))?;

        // Verify status exists
        let status_exists = task_statuses::Entity::find_by_id(status_id)
            .one(db)
            .await?
            .is_some();
        if !status_exists {
            return Err(ApiError::BadRequest("Status ID tidak ditemukan".into()));
        }

        // Verify priority exists if provided
        if let Some(p_id) = dto.priority_id {
            let priority_exists = task_priorities::Entity::find_by_id(p_id)
                .one(db)
                .await?
                .is_some();
            if !priority_exists {
                return Err(ApiError::BadRequest("Priority ID tidak ditemukan".into()));
            }
        }

        let due_date = if let Some(ref d) = dto.due_date {
            match DateTime::parse_from_rfc3339(d) {
                Ok(parsed) => Some(parsed),
                Err(_) => return Err(ApiError::BadRequest("Format due_date tidak valid".into())),
            }
        } else {
            None
        };

        let now = Utc::now().into();
        let new_task = tasks::ActiveModel {
            user_id: Set(user_id),
            title: Set(title),
            description: Set(dto.description),
            status_id: Set(status_id),
            priority_id: Set(dto.priority_id),
            progress_percentage: Set(dto.progress_percentage.unwrap_or(0)),
            due_date: Set(due_date),
            archived_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let task = new_task.insert(db).await?;

        // Link initial tags if provided
        for tag_id in dto.tag_ids {
            let link = task_tags::ActiveModel {
                task_id: Set(task.task_id),
                tag_id: Set(tag_id),
            };
            let _ = link.insert(db).await;
        }

        // Write activity log
        let log = activity_logs::ActiveModel {
            user_id: Set(user_id),
            action: Set("task.create".into()),
            entity_type: Set(Some("task".into())),
            entity_id: Set(Some(task.task_id)),
            timestamp: Set(now),
            ..Default::default()
        };
        let _ = log.insert(db).await;

        Ok(Self::to_task_response(db, task).await)
    }

    pub async fn get_task(user_id: i32, task_id: i32) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, _) = Self::check_access(db, user_id, task_id).await?;
        Ok(Self::to_task_response(db, task).await)
    }

    pub async fn update_task(
        user_id: i32,
        task_id: i32,
        dto: UpdateTaskRequest,
    ) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengedit task ini".into(),
            ));
        }

        let old_status_id = task.status_id;
        let old_priority_id = task.priority_id;

        let mut active: tasks::ActiveModel = task.into();
        let mut modified = false;

        if let Some(title) = dto.title {
            active.title = Set(title);
            modified = true;
        }

        if dto.description.is_some() {
            active.description = Set(dto.description);
            modified = true;
        }

        if let Some(new_status_id) = dto.status_id {
            if new_status_id != old_status_id {
                let status_exists = task_statuses::Entity::find_by_id(new_status_id)
                    .one(db)
                    .await?
                    .is_some();
                if !status_exists {
                    return Err(ApiError::BadRequest("Status ID tidak ditemukan".into()));
                }

                // Write to task_history
                let history = task_history::ActiveModel {
                    task_id: Set(task_id),
                    old_status_id: Set(Some(old_status_id)),
                    new_status_id: Set(new_status_id),
                    changed_at: Set(Utc::now().into()),
                    ..Default::default()
                };
                let _ = history.insert(db).await;

                active.status_id = Set(new_status_id);
                modified = true;
            }
        }

        if let Some(new_p_id) = dto.priority_id {
            if Some(new_p_id) != old_priority_id {
                let p_exists = task_priorities::Entity::find_by_id(new_p_id)
                    .one(db)
                    .await?
                    .is_some();
                if !p_exists {
                    return Err(ApiError::BadRequest("Priority ID tidak ditemukan".into()));
                }

                // Write to task_priority_history
                let history = task_priority_history::ActiveModel {
                    task_id: Set(task_id),
                    old_priority_id: Set(old_priority_id),
                    new_priority_id: Set(Some(new_p_id)),
                    changed_at: Set(Utc::now().into()),
                    ..Default::default()
                };
                let _ = history.insert(db).await;

                active.priority_id = Set(Some(new_p_id));
                modified = true;
            }
        }

        if let Some(ref d) = dto.due_date {
            match DateTime::parse_from_rfc3339(d) {
                Ok(parsed) => {
                    active.due_date = Set(Some(parsed));
                    modified = true;
                }
                Err(_) => return Err(ApiError::BadRequest("Format due_date tidak valid".into())),
            }
        }

        if modified {
            active.updated_at = Set(Utc::now().into());
            let updated = active.update(db).await?;
            Ok(Self::to_task_response(db, updated).await)
        } else {
            Self::get_task(user_id, task_id).await
        }
    }

    pub async fn delete_task(user_id: i32, task_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner {
            return Err(ApiError::Forbidden(
                "Hanya pemilik task yang dapat menghapus task".into(),
            ));
        }

        tasks::Entity::delete_by_id(task.task_id).exec(db).await?;

        // Write activity log
        let log = activity_logs::ActiveModel {
            user_id: Set(user_id),
            action: Set("task.delete".into()),
            entity_type: Set(Some("task".into())),
            entity_id: Set(Some(task_id)),
            timestamp: Set(Utc::now().into()),
            ..Default::default()
        };
        let _ = log.insert(db).await;

        Ok(())
    }

    pub async fn archive_task(user_id: i32, task_id: i32) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner {
            return Err(ApiError::Forbidden(
                "Hanya pemilik task yang dapat mengarsipkan task".into(),
            ));
        }

        let now = Utc::now().into();
        let mut active: tasks::ActiveModel = task.into();
        active.archived_at = Set(Some(now));
        active.updated_at = Set(now);

        let updated = active.update(db).await?;
        Ok(Self::to_task_response(db, updated).await)
    }

    pub async fn unarchive_task(user_id: i32, task_id: i32) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner {
            return Err(ApiError::Forbidden(
                "Hanya pemilik task yang dapat membatalkan arsip task".into(),
            ));
        }

        let mut active: tasks::ActiveModel = task.into();
        active.archived_at = Set(None);
        active.updated_at = Set(Utc::now().into());

        let updated = active.update(db).await?;
        Ok(Self::to_task_response(db, updated).await)
    }

    pub async fn update_progress(
        user_id: i32,
        task_id: i32,
        dto: UpdateProgressRequest,
    ) -> Result<TaskResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengedit progress task ini".into(),
            ));
        }

        let progress = dto
            .progress_percentage
            .ok_or_else(|| ApiError::BadRequest("progress_percentage wajib diisi".into()))?;

        if !(0..=100).contains(&progress) {
            return Err(ApiError::BadRequest(
                "progress_percentage harus di antara 0 dan 100".into(),
            ));
        }

        let mut active: tasks::ActiveModel = task.into();
        active.progress_percentage = Set(progress);
        active.updated_at = Set(Utc::now().into());

        let updated = active.update(db).await?;
        Ok(Self::to_task_response(db, updated).await)
    }
}
