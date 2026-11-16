use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::error::api_error::ApiError;
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
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_subtasks(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<CreateSubtaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_subtask(user.user_id, task_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let subtask_id = req.param::<i32>("subtask_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateSubtaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_subtask(user.user_id, subtask_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_subtask(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let subtask_id = req.param::<i32>("subtask_id").unwrap_or_default();

    match TaskService::delete_subtask(user.user_id, subtask_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Subtask berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Notes Handlers
// ====================================================

#[handler]
pub async fn list_notes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_notes(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<CreateNoteRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_note(user.user_id, task_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let note_id = req.param::<i32>("note_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateNoteRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_note(user.user_id, note_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_note(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let note_id = req.param::<i32>("note_id").unwrap_or_default();

    match TaskService::delete_note(user.user_id, note_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Catatan berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_note_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let note_id = req.param::<i32>("note_id").unwrap_or_default();

    match TaskService::get_note_history(user.user_id, note_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Comments Handlers
// ====================================================

#[handler]
pub async fn list_comments(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_comments(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<CreateCommentRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_comment(user.user_id, task_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let comment_id = req.param::<i32>("comment_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateCommentRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_comment(user.user_id, comment_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_comment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let comment_id = req.param::<i32>("comment_id").unwrap_or_default();

    match TaskService::delete_comment(user.user_id, comment_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Komentar berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Attachments Handlers
// ====================================================

#[handler]
pub async fn list_attachments(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_attachments(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upload_attachment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    // Check JSON body
    if let Ok(dto) = parse_and_validate::<CreateAttachmentRequest>(req).await {
        let file_url = match dto.file_url {
            Some(u) => u,
            None => {
                res.render(ApiError::BadRequest("file_url wajib diisi".into()));
                return;
            }
        };
        match TaskService::add_attachment(user.user_id, task_id, file_url, dto.file_type).await {
            Ok(data) => {
                res.status_code(StatusCode::CREATED);
                res.render(Json(ApiResponse::new(data)));
            }
            Err(err) => res.render(err),
        }
        return;
    }

    // Fallback multipart upload
    if let Some(file) = req.file("file").await {
        let file_url = format!("/uploads/{}", file.name().unwrap_or("attachment"));
        let file_type = file.content_type().map(|c| c.to_string());
        match TaskService::add_attachment(user.user_id, task_id, file_url, file_type).await {
            Ok(data) => {
                res.status_code(StatusCode::CREATED);
                res.render(Json(ApiResponse::new(data)));
            }
            Err(err) => res.render(err),
        }
        return;
    }

    res.render(ApiError::BadRequest("Lampiran (file_url atau file form-data) wajib disertakan".into()));
}

#[handler]
pub async fn delete_attachment(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let attachment_id = req.param::<i32>("attachment_id").unwrap_or_default();

    match TaskService::delete_attachment(user.user_id, attachment_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Lampiran berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Reminders Handlers
// ====================================================

#[handler]
pub async fn list_reminders(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_reminders(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<CreateReminderRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_reminder(user.user_id, task_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let reminder_id = req.param::<i32>("reminder_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateReminderRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::update_reminder(user.user_id, reminder_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_reminder(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let reminder_id = req.param::<i32>("reminder_id").unwrap_or_default();

    match TaskService::delete_reminder(user.user_id, reminder_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Pengingat berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Recurrence Handlers
// ====================================================

#[handler]
pub async fn get_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_recurrence(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upsert_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<RecurrenceRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::upsert_recurrence(user.user_id, task_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_recurrence(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::delete_recurrence(user.user_id, task_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Aturan perulangan berhasil dihapus"))),
        Err(err) => res.render(err),
    }
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

    match TaskService::list_locations().await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
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
    let dto = match parse_and_validate::<CreateLocationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_location(dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn list_task_locations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_task_locations(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn link_task_location(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let location_id = req.param::<i32>("location_id").unwrap_or_default();

    match TaskService::link_task_location(user.user_id, task_id, location_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Lokasi berhasil ditautkan ke task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn unlink_task_location(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let location_id = req.param::<i32>("location_id").unwrap_or_default();

    match TaskService::unlink_task_location(user.user_id, task_id, location_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Lokasi berhasil dilepas dari task"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Sharing Handlers
// ====================================================

#[handler]
pub async fn list_shares(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_shares(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upsert_share(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let target_user_id = req.param::<i32>("user_id").unwrap_or_default();
    let dto = match parse_and_validate::<ShareTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::upsert_share(user.user_id, task_id, target_user_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_share(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let target_user_id = req.param::<i32>("user_id").unwrap_or_default();

    match TaskService::delete_share(user.user_id, task_id, target_user_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Akses berbagi berhasil dicabut"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Votes Handlers
// ====================================================

#[handler]
pub async fn get_votes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_votes(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upsert_vote(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<VoteTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::upsert_vote(user.user_id, task_id, dto).await {
        Ok(_) => res.render(Json(ApiResponse::new("Vote berhasil dicatat"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_vote(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::delete_vote(user.user_id, task_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Vote berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Audit History Handlers
// ====================================================

#[handler]
pub async fn get_status_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_status_history(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_priority_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_priority_history(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_labels_history(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::get_labels_history(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Integrations Handlers
// ====================================================

#[handler]
pub async fn list_integrations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    match TaskService::list_integrations(user.user_id, task_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_integration(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let dto = match parse_and_validate::<CreateIntegrationRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match TaskService::create_integration(user.user_id, task_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_integration(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let integration_id = req.param::<i32>("integration_id").unwrap_or_default();

    match TaskService::delete_integration(user.user_id, integration_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Integrasi berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ====================================================
// Task Classification Handlers (Mount on Task)
// ====================================================

#[handler]
pub async fn attach_task_tag(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let tag_id = req.param::<i32>("tag_id").unwrap_or_default();

    match TaskService::attach_task_tag(user.user_id, task_id, tag_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Tag berhasil dipasang ke task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn detach_task_tag(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let tag_id = req.param::<i32>("tag_id").unwrap_or_default();

    match TaskService::detach_task_tag(user.user_id, task_id, tag_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Tag berhasil dilepas dari task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn attach_task_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let category_id = req.param::<i32>("category_id").unwrap_or_default();

    match TaskService::attach_task_category(user.user_id, task_id, category_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Kategori berhasil dipasang ke task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn detach_task_category(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let category_id = req.param::<i32>("category_id").unwrap_or_default();

    match TaskService::detach_task_category(user.user_id, task_id, category_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Kategori berhasil dilepas dari task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn attach_task_label(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let label_id = req.param::<i32>("label_id").unwrap_or_default();

    match TaskService::attach_task_label(user.user_id, task_id, label_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Label berhasil dipasang ke task"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn detach_task_label(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();
    let label_id = req.param::<i32>("label_id").unwrap_or_default();

    match TaskService::detach_task_label(user.user_id, task_id, label_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Label berhasil dilepas dari task"))),
        Err(err) => res.render(err),
    }
}
