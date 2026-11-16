use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::error::api_error::ApiError;
use crate::modules::classifications::dtos::*;

#[handler]
pub async fn list_tags(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_tag(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<TagRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Tag created")));
}

#[handler]
pub async fn update_tag(req: &mut Request, res: &mut Response) {
    let _tag_id: i64 = match req.param::<i64>("tag_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("tag_id tidak valid".into()));
            return;
        }
    };
    let _dto = match parse_and_validate::<TagRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Tag updated")));
}

#[handler]
pub async fn delete_tag(req: &mut Request, res: &mut Response) {
    let _tag_id: i64 = match req.param::<i64>("tag_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("tag_id tidak valid".into()));
            return;
        }
    };
    let _force = req.query::<bool>("force").unwrap_or(false);
    res.status_code(StatusCode::NO_CONTENT);
}

#[handler]
pub async fn list_categories(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_category(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<CategoryRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Category created")));
}

#[handler]
pub async fn update_category(req: &mut Request, res: &mut Response) {
    let _category_id: i64 = match req.param::<i64>("category_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("category_id tidak valid".into()));
            return;
        }
    };
    let _dto = match parse_and_validate::<CategoryRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Category updated")));
}

#[handler]
pub async fn delete_category(req: &mut Request, res: &mut Response) {
    let _category_id: i64 = match req.param::<i64>("category_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("category_id tidak valid".into()));
            return;
        }
    };
    let _force = req.query::<bool>("force").unwrap_or(false);
    res.status_code(StatusCode::NO_CONTENT);
}

#[handler]
pub async fn list_labels(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_label(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<LabelRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Label created")));
}

#[handler]
pub async fn update_label(req: &mut Request, res: &mut Response) {
    let _label_id: i64 = match req.param::<i64>("label_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("label_id tidak valid".into()));
            return;
        }
    };
    let _dto = match parse_and_validate::<LabelRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Label updated")));
}

#[handler]
pub async fn delete_label(req: &mut Request, res: &mut Response) {
    let _label_id: i64 = match req.param::<i64>("label_id") {
        Some(id) => id,
        None => {
            res.render(ApiError::BadRequest("label_id tidak valid".into()));
            return;
        }
    };
    let _force = req.query::<bool>("force").unwrap_or(false);
    res.status_code(StatusCode::NO_CONTENT);
}
