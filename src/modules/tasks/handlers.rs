use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::tasks::dtos::*;
use crate::modules::tasks::services::TaskService;

// ====================================================
// Core Tasks Handlers
// ====================================================

#[handler]
pub async fn list_tasks(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let query = req.parse_queries::<TaskListQuery>().unwrap_or(TaskListQuery {
        status_id: None,
        priority_id: None,
        tag_id: None,
        category_id: None,
        label_id: None,
        due_before: None,
        due_after: None,
        archived: None,
        scope: None,
        search: None,
        sort: None,
        include: None,
        page: None,
        per_page: None,
    });

    match TaskService::list_tasks(user.user_id, query).await {
        Ok(data) => res.render(Json(data)),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let dto = match parse_and_validate::<CreateTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_task(user.user_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(data));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_task(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    let dto = match parse_and_validate::<UpdateTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_task(user.user_id, task_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::delete_task(user.user_id, task_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Task berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn archive_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::archive_task(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn unarchive_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::unarchive_task(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    let dto = match parse_and_validate::<UpdateProgressRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_progress(user.user_id, task_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Subtasks Handlers
// ====================================================

#[handler]
pub async fn list_subtasks(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<CreateSubtaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Subtask created")));
}

#[handler]
pub async fn update_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _subtask_id = req.param::<i32>("subtask_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateSubtaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Subtask updated")));
}

#[handler]
pub async fn delete_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _subtask_id = req.param::<i32>("subtask_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Subtask deleted")));
}

// ====================================================
// Notes Handlers
// ====================================================

#[handler]
pub async fn list_notes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<CreateNoteRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Note created")));
}

#[handler]
pub async fn update_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _note_id = req.param::<i32>("note_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateNoteRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Note updated")));
}

#[handler]
pub async fn delete_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _note_id = req.param::<i32>("note_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Note deleted")));
}

#[handler]
pub async fn get_note_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _note_id = req.param::<i32>("note_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

// ====================================================
// Comments Handlers
// ====================================================

#[handler]
pub async fn list_comments(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<CreateCommentRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Comment created")));
}

#[handler]
pub async fn update_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _comment_id = req.param::<i32>("comment_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateCommentRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Comment updated")));
}

#[handler]
pub async fn delete_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _comment_id = req.param::<i32>("comment_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Comment deleted")));
}

// ====================================================
// Attachments Handlers
// ====================================================

#[handler]
pub async fn list_attachments(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn upload_attachment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Attachment uploaded")));
}

#[handler]
pub async fn delete_attachment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _attachment_id = req.param::<i32>("attachment_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Attachment deleted")));
}

// ====================================================
// Reminders Handlers
// ====================================================

#[handler]
pub async fn list_reminders(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<CreateReminderRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Reminder created")));
}

#[handler]
pub async fn update_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _reminder_id = req.param::<i32>("reminder_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateReminderRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Reminder updated")));
}

#[handler]
pub async fn delete_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _reminder_id = req.param::<i32>("reminder_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Reminder deleted")));
}

// ====================================================
// Recurrence Handlers
// ====================================================

#[handler]
pub async fn get_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Recurrence rule")));
}

#[handler]
pub async fn upsert_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<RecurrenceRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Recurrence set")));
}

#[handler]
pub async fn delete_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Recurrence deleted")));
}

// ====================================================
// Locations Handlers
// ====================================================

#[handler]
pub async fn list_locations(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_location(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _dto = match parse_and_validate::<CreateLocationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Location created")));
}

#[handler]
pub async fn list_task_locations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn link_task_location(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _location_id = req.param::<i32>("location_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Location linked to task")));
}

#[handler]
pub async fn unlink_task_location(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _location_id = req.param::<i32>("location_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Location unlinked from task")));
}

// ====================================================
// Sharing Handlers
// ====================================================

#[handler]
pub async fn list_shares(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn upsert_share(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _target_user_id = req.param::<i32>("user_id").unwrap_or_default();
    let _dto = match parse_and_validate::<ShareTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Share permission updated")));
}

#[handler]
pub async fn delete_share(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _target_user_id = req.param::<i32>("user_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Share removed")));
}

// ====================================================
// Votes Handlers
// ====================================================

#[handler]
pub async fn get_votes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(VoteSummaryResponse {
        up: 0,
        down: 0,
        my_vote: None,
    })));
}

#[handler]
pub async fn upsert_vote(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<VoteTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Vote recorded")));
}

#[handler]
pub async fn delete_vote(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Vote removed")));
}

// ====================================================
// History Handlers (Read-only)
// ====================================================

#[handler]
pub async fn get_status_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_priority_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_labels_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

// ====================================================
// External Integrations Handlers
// ====================================================

#[handler]
pub async fn list_integrations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_integration(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _dto = match parse_and_validate::<CreateIntegrationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Integration added")));
}

#[handler]
pub async fn delete_integration(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _integration_id = req.param::<i32>("integration_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Integration deleted")));
}

// ====================================================
// Task Classification Handlers (Mount on Task)
// ====================================================

#[handler]
pub async fn attach_task_tag(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _tag_id = req.param::<i32>("tag_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Tag attached to task")));
}

#[handler]
pub async fn detach_task_tag(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _tag_id = req.param::<i32>("tag_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Tag detached from task")));
}

#[handler]
pub async fn attach_task_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _category_id = req.param::<i32>("category_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Category attached to task")));
}

#[handler]
pub async fn detach_task_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _category_id = req.param::<i32>("category_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Category detached from task")));
}

#[handler]
pub async fn attach_task_label(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _label_id = req.param::<i32>("label_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Label attached to task")));
}

#[handler]
pub async fn detach_task_label(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();
    let _label_id = req.param::<i32>("label_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Label detached from task")));
}
