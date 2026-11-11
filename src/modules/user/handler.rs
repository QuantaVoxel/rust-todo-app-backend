use crate::core::errors::{ApiError, validate_json};
use crate::modules::user::dtos::{CreateUser, User};
use crate::modules::user::service::get_all_users;
use salvo::hyper::StatusCode;
use salvo::prelude::Json;
use salvo::{Request, Response, handler};

#[handler]
pub async fn get_all_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    let user: CreateUser = validate_json(req).await?;

    res.status_code(StatusCode::CREATED);
    res.render(Json(user));

    Ok(())
}
#[handler]
pub fn create_user(res: &mut Response) {}
#[handler]
pub fn get_user(res: &mut Response) {}
#[handler]
pub fn edit_user(res: &mut Response) {}
#[handler]
pub fn delete_user(res: &mut Response) {}
#[handler]
pub fn get_user_tasks(res: &mut Response) {}
#[handler]
pub fn get_user_task_comments(res: &mut Response) {}
#[handler]
pub fn get_user_task_sharings(res: &mut Response) {}
