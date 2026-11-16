use salvo::prelude::*;

use crate::error::ApiError;

/// Handler to catch 404 Not Found responses and return standardized JSON error.
#[handler]
pub async fn not_found_handler(
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    if res.status_code.is_none() || res.status_code == Some(StatusCode::NOT_FOUND) {
        res.status_code(StatusCode::NOT_FOUND);
        res.render(ApiError::NotFound("Route tidak ditemukan".to_string()));
        ctrl.skip_rest();
    }
}
