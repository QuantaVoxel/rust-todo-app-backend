use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
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
