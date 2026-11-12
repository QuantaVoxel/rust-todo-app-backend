use crate::core::errors::{ApiError, validate_json, validate_query};
use crate::core::pagination::PaginatedResponse;
use crate::infrastructure::db::get_db;
use crate::modules::user::dtos::{CreateUserDto, UserResponseDto};
use crate::modules::user::service;
use salvo::hyper::StatusCode;
use salvo::prelude::*;
use crate::core::filter::FilterQueryDto;

#[handler]
pub async fn get_all_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {

    // filter query
    let filter : FilterQueryDto = validate_query(req).await?;

    let db = get_db();
    let (users,meta) = service::get_all_users(db,filter).await?;

    let response_data: Vec<UserResponseDto> = users
        .into_iter()
        .map(|user| user.into())
        .collect();

    let final_response = PaginatedResponse::new(
        meta.current_page,
        meta.per_page,
        meta.total_pages,
        meta.total_items,
        response_data
    );

    res.status_code(StatusCode::OK);
    res.render(Json(final_response));

    Ok(())
}

#[handler]
pub async fn create_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    let payload: CreateUserDto = validate_json(req).await?;
    let db = get_db();

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
