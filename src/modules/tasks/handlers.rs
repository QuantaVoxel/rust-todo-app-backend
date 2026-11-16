use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::tasks::dtos::*;

#[handler]
pub async fn list_tasks(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
pub async fn create_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let _dto = match parse_and_validate::<CreateTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Task created")));
}

#[handler]
pub async fn get_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let task_id = req.param::<i32>("task_id").unwrap_or_default();

    res.render(Json(ApiResponse::new(format!("Task detail for ID {}", task_id))));
}

#[handler]
pub async fn update_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    let _dto = match parse_and_validate::<UpdateTaskRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Task updated")));
}

#[handler]
pub async fn delete_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    res.render(Json(ApiResponse::new("Task deleted")));
}

#[handler]
pub async fn archive_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    res.render(Json(ApiResponse::new("Task archived")));
}

#[handler]
pub async fn unarchive_task(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    res.render(Json(ApiResponse::new("Task unarchived")));
}

#[handler]
pub async fn update_progress(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _task_id = req.param::<i32>("task_id").unwrap_or_default();

    let _dto = match parse_and_validate::<UpdateProgressRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Progress updated")));
}
