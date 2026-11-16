use salvo::http::StatusCode;
use salvo::prelude::*;

use crate::error::ApiError;

/// Catcher middleware that catches all unhandled HTTP client (4xx) and server (5xx)
/// error status codes (e.g. 404 Not Found, 405 Method Not Allowed, 500 Server Error)
/// and formats them into the standard API JSON error response structure.
#[handler]
pub async fn error_catcher(
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let status = res.status_code.unwrap_or(StatusCode::NOT_FOUND);

    if status.is_client_error() || status.is_server_error() {
        let err = ApiError::from_status_code(status, None);
        res.status_code(status);
        res.render(err);
        ctrl.skip_rest();
    }
}

pub use error_catcher as not_found_handler;
