use crate::core::errors::{ApiError, validate_json, validate_query};
use crate::core::filter::FilterQueryDto;
use crate::core::pagination::PaginatedResponse;
use crate::infrastructure::db::get_db;
use crate::modules::user::dtos::{CreateUserDto, UpdateUserDto, UserResponseDto};
use crate::modules::user::service;
use salvo::hyper::StatusCode;
use salvo::prelude::*;
use sea_orm::prelude::Uuid;
use serde_json::json;

#[handler]
pub async fn get_all_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    // filter query
    let filter: FilterQueryDto = validate_query(req).await?;

    let db = get_db();
    let (users, meta) = service::get_all_users(db, filter).await?;

    let response_data: Vec<UserResponseDto> = users.into_iter().map(|user| user.into()).collect();

    let final_response = PaginatedResponse::new(
        meta.current_page,
        meta.per_page,
        meta.total_pages,
        meta.total_items,
        response_data,
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
    let user_response = user.map(UserResponseDto::from);

    res.status_code(StatusCode::CREATED);
    res.render(Json(user_response));

    Ok(())
}

#[handler]
pub async fn get_user_by_id(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    let db = get_db();
    // get user id from param
    let user_id = req
        .param::<Uuid>("user_id")
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    let user_model = service::get_user_by_id(db, user_id).await?;

    let response_data = user_model.map(UserResponseDto::from);

    res.status_code(StatusCode::OK);
    res.render(Json(response_data));

    Ok(())
}

#[handler]
pub async fn edit_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    let payload: UpdateUserDto = validate_json(req).await?;
    let db = get_db();

    let user_id = req
        .param::<Uuid>("user_id")
        .ok_or_else(|| ApiError::not_found(""))?;

    let updated_user = service::edit_user(db, user_id, payload).await?;

    let response_data: UserResponseDto = updated_user.into();

    res.status_code(StatusCode::OK);
    res.render(Json(response_data));

    Ok(())
}

#[handler]
pub async fn delete_user(req: &mut Request, res: &mut Response) -> Result<(), ApiError> {
    let user_id = req
        .param::<Uuid>("user_id")
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    let db = get_db();

    service::delete_user(db, user_id).await?;

    res.render(Json(json!({
        "message" : "Data user berhasil dihapus"
    })));

    Ok(())
}

#[handler]
pub async fn get_user_tasks(req: &mut Request, res: &mut Response) {
    todo!()
}

#[handler]
pub async fn get_user_task_comments(req: &mut Request, res: &mut Response) {
    todo!()
}

#[handler]
pub async fn get_user_task_sharings(req: &mut Request, res: &mut Response) {
    todo!()
}
