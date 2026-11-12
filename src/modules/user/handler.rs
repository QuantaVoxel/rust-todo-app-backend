use crate::core::errors::{ApiError, validate_json};
use crate::infrastructure::db;
use crate::modules::user::dtos::CreateUserDto;
use crate::modules::user::service;
use salvo::hyper::StatusCode;
use salvo::prelude::*;

#[handler]
pub async fn get_all_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    Ok(())
}
#[handler]
pub async fn create_user(
    req: &mut Request,
    res: &mut Response,
) -> Result<(), ApiError> {
    let payload: CreateUserDto = validate_json(req).await?;
    let db = db::get_db();

    let user = service::create_user(&db, payload).await?;

    res.status_code(StatusCode::CREATED);
    res.render(Json(user));

    Ok(())
}
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
