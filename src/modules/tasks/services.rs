use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::common::response::{PageMeta, PaginatedResponse};
use crate::core::db::connection::try_get_db_connection;
use crate::entity::sea_orm_active_enums::{
    RecurrenceFrequencyEnum, SharePermissionEnum, VoteTypeEnum,
};
use crate::entity::{
    activity_logs, external_apps, locations, notes, recurring_tasks, reminders, subtasks,
    tags, task_attachments, task_categories, task_category_assignment, task_comments,
    task_external_integration, task_history, task_label_assignment, task_labels,
    task_labels_history, task_location, task_notes_history, task_priorities,
    task_priority_history, task_sharing, task_statuses, task_tags, task_votes, tasks, users,
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

    // ====================================================
    // Subtasks
    // ====================================================

    pub async fn list_subtasks(user_id: i32, task_id: i32) -> Result<Vec<subtasks::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = subtasks::Entity::find()
            .filter(subtasks::Column::TaskId.eq(task_id))
            .order_by_asc(subtasks::Column::SubtaskId)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_subtask(
        user_id: i32,
        task_id: i32,
        dto: CreateSubtaskRequest,
    ) -> Result<subtasks::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menambahkan subtask pada task ini".into(),
            ));
        }

        let title = dto
            .title
            .ok_or_else(|| ApiError::BadRequest("title subtask wajib diisi".into()))?;
        let status_id = dto
            .status_id
            .ok_or_else(|| ApiError::BadRequest("status_id subtask wajib diisi".into()))?;

        let status_exists = task_statuses::Entity::find_by_id(status_id)
            .one(db)
            .await?
            .is_some();
        if !status_exists {
            return Err(ApiError::BadRequest("Status ID tidak ditemukan".into()));
        }

        let new_subtask = subtasks::ActiveModel {
            task_id: Set(task_id),
            title: Set(title),
            status_id: Set(status_id),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let created = new_subtask.insert(db).await?;
        Ok(created)
    }

    pub async fn update_subtask(
        user_id: i32,
        subtask_id: i32,
        dto: UpdateSubtaskRequest,
    ) -> Result<subtasks::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let subtask = subtasks::Entity::find_by_id(subtask_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Subtask tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, subtask.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah subtask ini".into(),
            ));
        }

        let mut active: subtasks::ActiveModel = subtask.into();
        if let Some(t) = dto.title {
            active.title = Set(t);
        }
        if let Some(s) = dto.status_id {
            let status_exists = task_statuses::Entity::find_by_id(s)
                .one(db)
                .await?
                .is_some();
            if !status_exists {
                return Err(ApiError::BadRequest("Status ID tidak ditemukan".into()));
            }
            active.status_id = Set(s);
        }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_subtask(user_id: i32, subtask_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let subtask = subtasks::Entity::find_by_id(subtask_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Subtask tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, subtask.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus subtask ini".into(),
            ));
        }

        subtasks::Entity::delete_by_id(subtask_id).exec(db).await?;
        Ok(())
    }

    // ====================================================
    // Notes & Note History
    // ====================================================

    pub async fn list_notes(user_id: i32, task_id: i32) -> Result<Vec<notes::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = notes::Entity::find()
            .filter(notes::Column::TaskId.eq(task_id))
            .order_by_asc(notes::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_note(
        user_id: i32,
        task_id: i32,
        dto: CreateNoteRequest,
    ) -> Result<notes::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menambahkan catatan".into(),
            ));
        }

        let content = dto
            .content
            .ok_or_else(|| ApiError::BadRequest("content catatan wajib diisi".into()))?;

        let new_note = notes::ActiveModel {
            task_id: Set(task_id),
            content: Set(content),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let created = new_note.insert(db).await?;
        Ok(created)
    }

    pub async fn update_note(
        user_id: i32,
        note_id: i32,
        dto: UpdateNoteRequest,
    ) -> Result<notes::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let note = notes::Entity::find_by_id(note_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Catatan tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, note.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah catatan ini".into(),
            ));
        }

        let new_content = dto
            .content
            .ok_or_else(|| ApiError::BadRequest("content catatan wajib diisi".into()))?;

        if note.content != new_content {
            let history = task_notes_history::ActiveModel {
                note_id: Set(note.note_id),
                old_content: Set(Some(note.content.clone())),
                new_content: Set(new_content.clone()),
                updated_at: Set(Utc::now().into()),
                ..Default::default()
            };
            let _ = history.insert(db).await?;
        }

        let mut active: notes::ActiveModel = note.into();
        active.content = Set(new_content);
        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_note(user_id: i32, note_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let note = notes::Entity::find_by_id(note_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Catatan tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, note.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus catatan ini".into(),
            ));
        }

        notes::Entity::delete_by_id(note_id).exec(db).await?;
        Ok(())
    }

    pub async fn get_note_history(
        user_id: i32,
        note_id: i32,
    ) -> Result<Vec<task_notes_history::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let note = notes::Entity::find_by_id(note_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Catatan tidak ditemukan".into()))?;

        Self::check_access(db, user_id, note.task_id).await?;

        let history = task_notes_history::Entity::find()
            .filter(task_notes_history::Column::NoteId.eq(note_id))
            .order_by_desc(task_notes_history::Column::UpdatedAt)
            .all(db)
            .await?;
        Ok(history)
    }

    // ====================================================
    // Comments
    // ====================================================

    pub async fn list_comments(user_id: i32, task_id: i32) -> Result<Vec<task_comments::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_comments::Entity::find()
            .filter(task_comments::Column::TaskId.eq(task_id))
            .order_by_asc(task_comments::Column::CreatedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_comment(
        user_id: i32,
        task_id: i32,
        dto: CreateCommentRequest,
    ) -> Result<task_comments::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level == TaskAccessLevel::View {
            return Err(ApiError::Forbidden(
                "Hanya user dengan izin comment atau edit yang dapat berkomentar".into(),
            ));
        }

        let comment_text = dto
            .comment
            .ok_or_else(|| ApiError::BadRequest("comment wajib diisi".into()))?;

        let new_comment = task_comments::ActiveModel {
            task_id: Set(task_id),
            user_id: Set(user_id),
            comment: Set(comment_text),
            created_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let created = new_comment.insert(db).await?;
        Ok(created)
    }

    pub async fn update_comment(
        user_id: i32,
        comment_id: i32,
        dto: UpdateCommentRequest,
    ) -> Result<task_comments::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let comment = task_comments::Entity::find_by_id(comment_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Komentar tidak ditemukan".into()))?;

        if comment.user_id != user_id {
            return Err(ApiError::Forbidden(
                "Hanya pembuat komentar yang dapat mengedit komentar".into(),
            ));
        }

        let comment_text = dto
            .comment
            .ok_or_else(|| ApiError::BadRequest("comment wajib diisi".into()))?;

        let mut active: task_comments::ActiveModel = comment.into();
        active.comment = Set(comment_text);
        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_comment(user_id: i32, comment_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let comment = task_comments::Entity::find_by_id(comment_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Komentar tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, comment.task_id).await?;
        if comment.user_id != user_id && level != TaskAccessLevel::Owner {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus komentar ini".into(),
            ));
        }

        task_comments::Entity::delete_by_id(comment_id).exec(db).await?;
        Ok(())
    }

    // ====================================================
    // Attachments
    // ====================================================

    pub async fn list_attachments(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_attachments::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_attachments::Entity::find()
            .filter(task_attachments::Column::TaskId.eq(task_id))
            .order_by_asc(task_attachments::Column::UploadedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn add_attachment(
        user_id: i32,
        task_id: i32,
        file_url: String,
        file_type: Option<String>,
    ) -> Result<task_attachments::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengunggah lampiran".into(),
            ));
        }

        let new_att = task_attachments::ActiveModel {
            task_id: Set(task_id),
            file_url: Set(file_url),
            file_type: Set(file_type),
            uploaded_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let created = new_att.insert(db).await?;
        Ok(created)
    }

    pub async fn delete_attachment(user_id: i32, attachment_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let att = task_attachments::Entity::find_by_id(attachment_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Lampiran tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, att.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus lampiran ini".into(),
            ));
        }

        task_attachments::Entity::delete_by_id(attachment_id).exec(db).await?;
        Ok(())
    }

    // ====================================================
    // Reminders
    // ====================================================

    pub async fn list_reminders(user_id: i32, task_id: i32) -> Result<Vec<reminders::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = reminders::Entity::find()
            .filter(reminders::Column::TaskId.eq(task_id))
            .order_by_asc(reminders::Column::ReminderTime)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_reminder(
        user_id: i32,
        task_id: i32,
        dto: CreateReminderRequest,
    ) -> Result<reminders::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk memasang pengingat".into(),
            ));
        }

        let time_str = dto
            .reminder_time
            .ok_or_else(|| ApiError::BadRequest("reminder_time wajib diisi".into()))?;
        let reminder_time = DateTime::parse_from_rfc3339(&time_str)
            .map_err(|_| ApiError::BadRequest("Format reminder_time tidak valid".into()))?;

        let new_reminder = reminders::ActiveModel {
            task_id: Set(task_id),
            reminder_time: Set(reminder_time),
            notification_sent: Set(false),
            ..Default::default()
        };

        let created = new_reminder.insert(db).await?;
        Ok(created)
    }

    pub async fn update_reminder(
        user_id: i32,
        reminder_id: i32,
        dto: UpdateReminderRequest,
    ) -> Result<reminders::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let reminder = reminders::Entity::find_by_id(reminder_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Pengingat tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, reminder.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah pengingat ini".into(),
            ));
        }

        let time_str = dto
            .reminder_time
            .ok_or_else(|| ApiError::BadRequest("reminder_time wajib diisi".into()))?;
        let reminder_time = DateTime::parse_from_rfc3339(&time_str)
            .map_err(|_| ApiError::BadRequest("Format reminder_time tidak valid".into()))?;

        let mut active: reminders::ActiveModel = reminder.into();
        active.reminder_time = Set(reminder_time);
        active.notification_sent = Set(false);

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_reminder(user_id: i32, reminder_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let reminder = reminders::Entity::find_by_id(reminder_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Pengingat tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, reminder.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus pengingat ini".into(),
            ));
        }

        reminders::Entity::delete_by_id(reminder_id).exec(db).await?;
        Ok(())
    }

    // ====================================================
    // Recurrence
    // ====================================================

    pub async fn get_recurrence(
        user_id: i32,
        task_id: i32,
    ) -> Result<Option<recurring_tasks::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let item = recurring_tasks::Entity::find()
            .filter(recurring_tasks::Column::TaskId.eq(task_id))
            .one(db)
            .await?;
        Ok(item)
    }

    pub async fn upsert_recurrence(
        user_id: i32,
        task_id: i32,
        dto: RecurrenceRequest,
    ) -> Result<recurring_tasks::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengatur perulangan task ini".into(),
            ));
        }

        let freq_str = dto
            .frequency
            .ok_or_else(|| ApiError::BadRequest("frequency wajib diisi".into()))?;
        let frequency = match freq_str.as_str() {
            "daily" => RecurrenceFrequencyEnum::Daily,
            "weekly" => RecurrenceFrequencyEnum::Weekly,
            "monthly" => RecurrenceFrequencyEnum::Monthly,
            "yearly" => RecurrenceFrequencyEnum::Yearly,
            _ => return Err(ApiError::BadRequest("Frequency tidak valid".into())),
        };

        let next_due_str = dto
            .next_due_date
            .ok_or_else(|| ApiError::BadRequest("next_due_date wajib diisi".into()))?;
        let next_due_date = DateTime::parse_from_rfc3339(&next_due_str)
            .map_err(|_| ApiError::BadRequest("Format next_due_date tidak valid".into()))?;

        let existing = recurring_tasks::Entity::find()
            .filter(recurring_tasks::Column::TaskId.eq(task_id))
            .one(db)
            .await?;

        match existing {
            Some(ex) => {
                let mut active: recurring_tasks::ActiveModel = ex.into();
                active.frequency = Set(frequency);
                active.next_due_date = Set(next_due_date);
                let updated = active.update(db).await?;
                Ok(updated)
            }
            None => {
                let new_rec = recurring_tasks::ActiveModel {
                    task_id: Set(task_id),
                    frequency: Set(frequency),
                    next_due_date: Set(next_due_date),
                    ..Default::default()
                };
                let created = new_rec.insert(db).await?;
                Ok(created)
            }
        }
    }

    pub async fn delete_recurrence(user_id: i32, task_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus perulangan task ini".into(),
            ));
        }

        recurring_tasks::Entity::delete_many()
            .filter(recurring_tasks::Column::TaskId.eq(task_id))
            .exec(db)
            .await?;
        Ok(())
    }

    // ====================================================
    // Locations & Task Location
    // ====================================================

    pub async fn list_locations() -> Result<Vec<locations::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let items = locations::Entity::find()
            .order_by_asc(locations::Column::LocationId)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_location(dto: CreateLocationRequest) -> Result<locations::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;

        let location_name = dto
            .location_name
            .ok_or_else(|| ApiError::BadRequest("location_name wajib diisi".into()))?;
        let latitude = dto
            .latitude
            .ok_or_else(|| ApiError::BadRequest("latitude wajib diisi".into()))?;
        let longitude = dto
            .longitude
            .ok_or_else(|| ApiError::BadRequest("longitude wajib diisi".into()))?;

        let existing = locations::Entity::find()
            .filter(
                Condition::all()
                    .add(locations::Column::Latitude.eq(latitude))
                    .add(locations::Column::Longitude.eq(longitude)),
            )
            .one(db)
            .await?;

        if let Some(loc) = existing {
            return Ok(loc);
        }

        let new_loc = locations::ActiveModel {
            location_name: Set(location_name),
            latitude: Set(latitude),
            longitude: Set(longitude),
            ..Default::default()
        };

        let created = new_loc.insert(db).await?;
        Ok(created)
    }

    pub async fn list_task_locations(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<locations::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let links = task_location::Entity::find()
            .filter(task_location::Column::TaskId.eq(task_id))
            .all(db)
            .await?;

        let loc_ids: Vec<i32> = links.into_iter().map(|l| l.location_id).collect();
        if loc_ids.is_empty() {
            return Ok(Vec::new());
        }

        let items = locations::Entity::find()
            .filter(locations::Column::LocationId.is_in(loc_ids))
            .order_by_asc(locations::Column::LocationId)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn link_task_location(
        user_id: i32,
        task_id: i32,
        location_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menautkan lokasi ke task".into(),
            ));
        }

        let loc_exists = locations::Entity::find_by_id(location_id)
            .one(db)
            .await?
            .is_some();
        if !loc_exists {
            return Err(ApiError::NotFound("Lokasi tidak ditemukan".into()));
        }

        let already_linked = task_location::Entity::find()
            .filter(
                Condition::all()
                    .add(task_location::Column::TaskId.eq(task_id))
                    .add(task_location::Column::LocationId.eq(location_id)),
            )
            .one(db)
            .await?
            .is_some();

        if !already_linked {
            let link = task_location::ActiveModel {
                task_id: Set(task_id),
                location_id: Set(location_id),
            };
            let _ = link.insert(db).await?;
        }

        Ok(())
    }

    pub async fn unlink_task_location(
        user_id: i32,
        task_id: i32,
        location_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk melepas lokasi dari task".into(),
            ));
        }

        task_location::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_location::Column::TaskId.eq(task_id))
                    .add(task_location::Column::LocationId.eq(location_id)),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    // ====================================================
    // Sharing & Collaboration
    // ====================================================

    pub async fn list_shares(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_sharing::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let shares = task_sharing::Entity::find()
            .filter(task_sharing::Column::TaskId.eq(task_id))
            .all(db)
            .await?;
        Ok(shares)
    }

    pub async fn upsert_share(
        user_id: i32,
        task_id: i32,
        target_user_id: i32,
        dto: ShareTaskRequest,
    ) -> Result<task_sharing::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (task, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner {
            return Err(ApiError::Forbidden(
                "Hanya pemilik task yang dapat mengelola pembagian akses".into(),
            ));
        }

        if target_user_id == user_id || target_user_id == task.user_id {
            return Err(ApiError::BadRequest(
                "Tidak dapat membagikan task kepada diri sendiri".into(),
            ));
        }

        let user_exists = users::Entity::find_by_id(target_user_id)
            .one(db)
            .await?
            .is_some();
        if !user_exists {
            return Err(ApiError::NotFound("Pengguna tujuan tidak ditemukan".into()));
        }

        let perm_str = dto
            .permission
            .ok_or_else(|| ApiError::BadRequest("permission wajib diisi".into()))?;
        let permission = match perm_str.as_str() {
            "view" => SharePermissionEnum::View,
            "comment" => SharePermissionEnum::Comment,
            "edit" => SharePermissionEnum::Edit,
            _ => return Err(ApiError::BadRequest("Permission tidak valid".into())),
        };

        let existing = task_sharing::Entity::find()
            .filter(
                Condition::all()
                    .add(task_sharing::Column::TaskId.eq(task_id))
                    .add(task_sharing::Column::UserId.eq(target_user_id)),
            )
            .one(db)
            .await?;

        match existing {
            Some(ex) => {
                let mut active: task_sharing::ActiveModel = ex.into();
                active.permission = Set(permission);
                let updated = active.update(db).await?;
                Ok(updated)
            }
            None => {
                let new_share = task_sharing::ActiveModel {
                    task_id: Set(task_id),
                    user_id: Set(target_user_id),
                    permission: Set(permission),
                };
                let created = new_share.insert(db).await?;
                Ok(created)
            }
        }
    }

    pub async fn delete_share(
        user_id: i32,
        task_id: i32,
        target_user_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && user_id != target_user_id {
            return Err(ApiError::Forbidden(
                "Hanya pemilik task atau user terkait yang dapat mencabut pembagian".into(),
            ));
        }

        task_sharing::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_sharing::Column::TaskId.eq(task_id))
                    .add(task_sharing::Column::UserId.eq(target_user_id)),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    // ====================================================
    // Votes
    // ====================================================

    pub async fn get_votes(user_id: i32, task_id: i32) -> Result<VoteSummaryResponse, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let up_count = task_votes::Entity::find()
            .filter(
                Condition::all()
                    .add(task_votes::Column::TaskId.eq(task_id))
                    .add(task_votes::Column::VoteType.eq(VoteTypeEnum::Up)),
            )
            .count(db)
            .await? as i64;

        let down_count = task_votes::Entity::find()
            .filter(
                Condition::all()
                    .add(task_votes::Column::TaskId.eq(task_id))
                    .add(task_votes::Column::VoteType.eq(VoteTypeEnum::Down)),
            )
            .count(db)
            .await? as i64;

        let my_vote_row = task_votes::Entity::find()
            .filter(
                Condition::all()
                    .add(task_votes::Column::TaskId.eq(task_id))
                    .add(task_votes::Column::UserId.eq(user_id)),
            )
            .one(db)
            .await?;

        let my_vote = my_vote_row.map(|v| match v.vote_type {
            VoteTypeEnum::Up => "up".to_string(),
            VoteTypeEnum::Down => "down".to_string(),
        });

        Ok(VoteSummaryResponse {
            up: up_count,
            down: down_count,
            my_vote,
        })
    }

    pub async fn upsert_vote(
        user_id: i32,
        task_id: i32,
        dto: VoteTaskRequest,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let vote_str = dto
            .vote_type
            .ok_or_else(|| ApiError::BadRequest("vote_type wajib diisi".into()))?;
        let vote_type = match vote_str.as_str() {
            "up" => VoteTypeEnum::Up,
            "down" => VoteTypeEnum::Down,
            _ => return Err(ApiError::BadRequest("vote_type harus 'up' atau 'down'".into())),
        };

        let existing = task_votes::Entity::find()
            .filter(
                Condition::all()
                    .add(task_votes::Column::TaskId.eq(task_id))
                    .add(task_votes::Column::UserId.eq(user_id)),
            )
            .one(db)
            .await?;

        match existing {
            Some(ex) => {
                let mut active: task_votes::ActiveModel = ex.into();
                active.vote_type = Set(vote_type);
                let _ = active.update(db).await?;
            }
            None => {
                let new_vote = task_votes::ActiveModel {
                    task_id: Set(task_id),
                    user_id: Set(user_id),
                    vote_type: Set(vote_type),
                    ..Default::default()
                };
                let _ = new_vote.insert(db).await?;
            }
        }

        Ok(())
    }

    pub async fn delete_vote(user_id: i32, task_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        task_votes::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_votes::Column::TaskId.eq(task_id))
                    .add(task_votes::Column::UserId.eq(user_id)),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    // ====================================================
    // Audit History
    // ====================================================

    pub async fn get_status_history(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_history::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_history::Entity::find()
            .filter(task_history::Column::TaskId.eq(task_id))
            .order_by_desc(task_history::Column::ChangedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn get_priority_history(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_priority_history::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_priority_history::Entity::find()
            .filter(task_priority_history::Column::TaskId.eq(task_id))
            .order_by_desc(task_priority_history::Column::ChangedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn get_labels_history(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_labels_history::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_labels_history::Entity::find()
            .filter(task_labels_history::Column::TaskId.eq(task_id))
            .order_by_desc(task_labels_history::Column::ChangedAt)
            .all(db)
            .await?;
        Ok(items)
    }

    // ====================================================
    // Integrations
    // ====================================================

    pub async fn list_integrations(
        user_id: i32,
        task_id: i32,
    ) -> Result<Vec<task_external_integration::Model>, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        Self::check_access(db, user_id, task_id).await?;

        let items = task_external_integration::Entity::find()
            .filter(task_external_integration::Column::TaskId.eq(task_id))
            .all(db)
            .await?;
        Ok(items)
    }

    pub async fn create_integration(
        user_id: i32,
        task_id: i32,
        dto: CreateIntegrationRequest,
    ) -> Result<task_external_integration::Model, ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menambahkan integrasi pada task ini".into(),
            ));
        }

        let app_id = dto
            .app_id
            .ok_or_else(|| ApiError::BadRequest("app_id wajib diisi".into()))?;
        let external_task_id = dto
            .external_app_task_id
            .ok_or_else(|| ApiError::BadRequest("external_app_task_id wajib diisi".into()))?;

        let app_exists = external_apps::Entity::find_by_id(app_id)
            .one(db)
            .await?
            .is_some();
        if !app_exists {
            return Err(ApiError::BadRequest("External app tidak ditemukan".into()));
        }

        let new_integ = task_external_integration::ActiveModel {
            task_id: Set(task_id),
            app_id: Set(app_id),
            external_app_task_id: Set(external_task_id),
            ..Default::default()
        };

        let created = new_integ.insert(db).await?;
        Ok(created)
    }

    pub async fn delete_integration(user_id: i32, integration_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let integ = task_external_integration::Entity::find_by_id(integration_id)
            .one(db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Integrasi tidak ditemukan".into()))?;

        let (_, level) = Self::check_access(db, user_id, integ.task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk menghapus integrasi ini".into(),
            ));
        }

        task_external_integration::Entity::delete_by_id(integration_id)
            .exec(db)
            .await?;
        Ok(())
    }

    // ====================================================
    // Classification Junctions (Tags, Categories, Labels)
    // ====================================================

    pub async fn attach_task_tag(user_id: i32, task_id: i32, tag_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah tag task".into(),
            ));
        }

        let tag_exists = tags::Entity::find_by_id(tag_id).one(db).await?.is_some();
        if !tag_exists {
            return Err(ApiError::NotFound("Tag tidak ditemukan".into()));
        }

        let exists = task_tags::Entity::find()
            .filter(
                Condition::all()
                    .add(task_tags::Column::TaskId.eq(task_id))
                    .add(task_tags::Column::TagId.eq(tag_id)),
            )
            .one(db)
            .await?
            .is_some();

        if !exists {
            let link = task_tags::ActiveModel {
                task_id: Set(task_id),
                tag_id: Set(tag_id),
            };
            let _ = link.insert(db).await?;
        }

        Ok(())
    }

    pub async fn detach_task_tag(user_id: i32, task_id: i32, tag_id: i32) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah tag task".into(),
            ));
        }

        task_tags::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_tags::Column::TaskId.eq(task_id))
                    .add(task_tags::Column::TagId.eq(tag_id)),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn attach_task_category(
        user_id: i32,
        task_id: i32,
        category_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah kategori task".into(),
            ));
        }

        let cat_exists = task_categories::Entity::find_by_id(category_id)
            .one(db)
            .await?
            .is_some();
        if !cat_exists {
            return Err(ApiError::NotFound("Kategori tidak ditemukan".into()));
        }

        let exists = task_category_assignment::Entity::find()
            .filter(
                Condition::all()
                    .add(task_category_assignment::Column::TaskId.eq(task_id))
                    .add(task_category_assignment::Column::CategoryId.eq(category_id)),
            )
            .one(db)
            .await?
            .is_some();

        if !exists {
            let link = task_category_assignment::ActiveModel {
                task_id: Set(task_id),
                category_id: Set(category_id),
            };
            let _ = link.insert(db).await?;
        }

        Ok(())
    }

    pub async fn detach_task_category(
        user_id: i32,
        task_id: i32,
        category_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah kategori task".into(),
            ));
        }

        task_category_assignment::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_category_assignment::Column::TaskId.eq(task_id))
                    .add(task_category_assignment::Column::CategoryId.eq(category_id)),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn attach_task_label(
        user_id: i32,
        task_id: i32,
        label_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah label task".into(),
            ));
        }

        let label_exists = task_labels::Entity::find_by_id(label_id)
            .one(db)
            .await?
            .is_some();
        if !label_exists {
            return Err(ApiError::NotFound("Label tidak ditemukan".into()));
        }

        let exists = task_label_assignment::Entity::find()
            .filter(
                Condition::all()
                    .add(task_label_assignment::Column::TaskId.eq(task_id))
                    .add(task_label_assignment::Column::LabelId.eq(label_id)),
            )
            .one(db)
            .await?
            .is_some();

        if !exists {
            let link = task_label_assignment::ActiveModel {
                task_id: Set(task_id),
                label_id: Set(label_id),
            };
            let _ = link.insert(db).await?;

            let history = task_labels_history::ActiveModel {
                task_id: Set(task_id),
                old_label_id: Set(None),
                new_label_id: Set(Some(label_id)),
                changed_at: Set(Utc::now().into()),
                ..Default::default()
            };
            let _ = history.insert(db).await?;
        }

        Ok(())
    }

    pub async fn detach_task_label(
        user_id: i32,
        task_id: i32,
        label_id: i32,
    ) -> Result<(), ApiError> {
        let db = try_get_db_connection()
            .ok_or_else(|| ApiError::Internal("Database belum terhubung".into()))?;
        let (_, level) = Self::check_access(db, user_id, task_id).await?;
        if level != TaskAccessLevel::Owner && level != TaskAccessLevel::Edit {
            return Err(ApiError::Forbidden(
                "Tidak memiliki izin untuk mengubah label task".into(),
            ));
        }

        let deleted = task_label_assignment::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(task_label_assignment::Column::TaskId.eq(task_id))
                    .add(task_label_assignment::Column::LabelId.eq(label_id)),
            )
            .exec(db)
            .await?;

        if deleted.rows_affected > 0 {
            let history = task_labels_history::ActiveModel {
                task_id: Set(task_id),
                old_label_id: Set(Some(label_id)),
                new_label_id: Set(None),
                changed_at: Set(Utc::now().into()),
                ..Default::default()
            };
            let _ = history.insert(db).await?;
        }

        Ok(())
    }
}
