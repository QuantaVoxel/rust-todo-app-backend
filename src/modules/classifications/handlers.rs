use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::modules::classifications::dtos::*;
use crate::modules::classifications::services::ClassificationService;

// ----------------------------------------------------
// Tags Handlers
// ----------------------------------------------------

#[endpoint(tags("Tags"), summary = "List all tags", security(("bearer_auth" = [])))]
pub async fn list_tags(_req: &mut Request, res: &mut Response) {
    match ClassificationService::list_tags().await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Tags"), summary = "Create new tag", security(("bearer_auth" = [])))]
pub async fn create_tag(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<TagRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::create_tag(dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Tags"), summary = "Update tag by ID", security(("bearer_auth" = [])))]
pub async fn update_tag(req: &mut Request, res: &mut Response) {
    let tag_id = req.param::<i32>("tag_id").unwrap_or_default();
    let dto = match parse_and_validate::<TagRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::update_tag(tag_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Tags"), summary = "Delete tag by ID (?force=true for cascading)", security(("bearer_auth" = [])))]
pub async fn delete_tag(req: &mut Request, res: &mut Response) {
    let tag_id = req.param::<i32>("tag_id").unwrap_or_default();
    let force = req.query::<bool>("force").unwrap_or(false);

    match ClassificationService::delete_tag(tag_id, force).await {
        Ok(_) => res.render(Json(ApiResponse::new("Tag berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ----------------------------------------------------
// Categories Handlers
// ----------------------------------------------------

#[endpoint(tags("Categories"), summary = "List all categories", security(("bearer_auth" = [])))]
pub async fn list_categories(_req: &mut Request, res: &mut Response) {
    match ClassificationService::list_categories().await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Categories"), summary = "Create new category", security(("bearer_auth" = [])))]
pub async fn create_category(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<CategoryRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::create_category(dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Categories"), summary = "Update category by ID", security(("bearer_auth" = [])))]
pub async fn update_category(req: &mut Request, res: &mut Response) {
    let category_id = req.param::<i32>("category_id").unwrap_or_default();
    let dto = match parse_and_validate::<CategoryRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::update_category(category_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Categories"), summary = "Delete category by ID (?force=true for cascading)", security(("bearer_auth" = [])))]
pub async fn delete_category(req: &mut Request, res: &mut Response) {
    let category_id = req.param::<i32>("category_id").unwrap_or_default();
    let force = req.query::<bool>("force").unwrap_or(false);

    match ClassificationService::delete_category(category_id, force).await {
        Ok(_) => res.render(Json(ApiResponse::new("Kategori berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

// ----------------------------------------------------
// Labels Handlers
// ----------------------------------------------------

#[endpoint(tags("Labels"), summary = "List all labels", security(("bearer_auth" = [])))]
pub async fn list_labels(_req: &mut Request, res: &mut Response) {
    match ClassificationService::list_labels().await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Labels"), summary = "Create new label", security(("bearer_auth" = [])))]
pub async fn create_label(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<LabelRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::create_label(dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Labels"), summary = "Update label by ID", security(("bearer_auth" = [])))]
pub async fn update_label(req: &mut Request, res: &mut Response) {
    let label_id = req.param::<i32>("label_id").unwrap_or_default();
    let dto = match parse_and_validate::<LabelRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ClassificationService::update_label(label_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[endpoint(tags("Labels"), summary = "Delete label by ID (?force=true for cascading)", security(("bearer_auth" = [])))]
pub async fn delete_label(req: &mut Request, res: &mut Response) {
    let label_id = req.param::<i32>("label_id").unwrap_or_default();
    let force = req.query::<bool>("force").unwrap_or(false);

    match ClassificationService::delete_label(label_id, force).await {
        Ok(_) => res.render(Json(ApiResponse::new("Label berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}
