use salvo::prelude::*;

use crate::common::response::ApiResponse;

#[handler]
pub async fn get_statuses(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_priorities(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_external_apps(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn get_enums(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new(serde_json::json!({
        "vote_type": ["up", "down"],
        "frequency": ["daily", "weekly", "monthly", "yearly"],
        "permission": ["view", "comment", "edit"],
        "connection_status": ["pending", "accepted", "blocked"],
        "notification_type": ["reminder", "comment", "share", "system"]
    }))));
}
