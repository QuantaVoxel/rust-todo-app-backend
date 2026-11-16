use scrutiny::Validate;
use serde::Deserialize;

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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSubtaskRequest {
    #[validate(required, min = 1, max = 255)]
    pub title: Option<String>,

    #[validate(required)]
    pub status_id: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(required)]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(required)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReminderRequest {
    #[validate(required, datetime)]
    pub reminder_time: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecurrenceRequest {
    #[validate(required, in_list("daily", "weekly", "monthly", "yearly"))]
    pub frequency: Option<String>,

    #[validate(required, datetime)]
    pub next_due_date: Option<String>,
}

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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[validate(required, max = 255)]
    pub location_name: Option<String>,

    #[validate(required)]
    pub latitude: Option<f64>,

    #[validate(required)]
    pub longitude: Option<f64>,
}
