use scrutiny::Validate;
use serde::{Deserialize, Serialize};

// ----------------------------------------------------
// Task DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(required, min = 1, max = 255)]
    pub title: Option<String>,

    pub description: Option<String>,

    #[validate(required)]
    pub status_id: Option<i32>,

    pub priority_id: Option<i32>,

    #[validate(sometimes, between(min = 0, max = 100))]
    pub progress_percentage: Option<i32>,

    #[validate(sometimes, datetime)]
    pub due_date: Option<String>,

    #[serde(default)]
    pub tag_ids: Vec<i32>,

    #[serde(default)]
    pub category_ids: Vec<i32>,

    #[serde(default)]
    pub label_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(sometimes, min = 1, max = 255)]
    pub title: Option<String>,

    pub description: Option<String>,
    pub status_id: Option<i32>,
    pub priority_id: Option<i32>,

    #[validate(sometimes, datetime)]
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProgressRequest {
    #[validate(required, between(min = 0, max = 100))]
    pub progress_percentage: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TaskListQuery {
    pub status_id: Option<i32>,
    pub priority_id: Option<i32>,
    pub tag_id: Option<i32>,
    pub category_id: Option<i32>,
    pub label_id: Option<i32>,
    pub due_before: Option<String>,
    pub due_after: Option<String>,
    pub archived: Option<bool>,
    pub scope: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub include: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

// ----------------------------------------------------
// Subtask DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSubtaskRequest {
    #[validate(required, min = 1, max = 255)]
    pub title: Option<String>,

    #[validate(required)]
    pub status_id: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSubtaskRequest {
    #[validate(sometimes, min = 1, max = 255)]
    pub title: Option<String>,

    pub status_id: Option<i32>,
}

// ----------------------------------------------------
// Note DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(required)]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteRequest {
    #[validate(required)]
    pub content: Option<String>,
}

// ----------------------------------------------------
// Comment DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(required)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(required)]
    pub comment: Option<String>,
}

// ----------------------------------------------------
// Reminder DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReminderRequest {
    #[validate(required, datetime)]
    pub reminder_time: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateReminderRequest {
    #[validate(required, datetime)]
    pub reminder_time: Option<String>,
}

// ----------------------------------------------------
// Recurrence DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct RecurrenceRequest {
    #[validate(required, in_list("daily", "weekly", "monthly", "yearly"))]
    pub frequency: Option<String>,

    #[validate(required, datetime)]
    pub next_due_date: Option<String>,
}

// ----------------------------------------------------
// Location DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[validate(required, max = 255)]
    pub location_name: Option<String>,

    #[validate(required)]
    pub latitude: Option<f64>,

    #[validate(required)]
    pub longitude: Option<f64>,
}

// ----------------------------------------------------
// Sharing & Vote DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct ShareTaskRequest {
    #[validate(required, in_list("view", "comment", "edit"))]
    pub permission: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VoteTaskRequest {
    #[validate(required, in_list("up", "down"))]
    pub vote_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteSummaryResponse {
    pub up: i64,
    pub down: i64,
    pub my_vote: Option<String>,
}

// ----------------------------------------------------
// Integration DTOs
// ----------------------------------------------------

#[derive(Debug, Deserialize, Validate)]
pub struct CreateIntegrationRequest {
    #[validate(required)]
    pub app_id: Option<i32>,

    #[validate(required, max = 255)]
    pub external_app_task_id: Option<String>,
}
