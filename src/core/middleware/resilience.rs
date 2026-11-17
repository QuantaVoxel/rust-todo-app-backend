use std::time::Duration;
use salvo::compression::Compression;
use salvo::http::StatusCode;
use salvo::prelude::*;

use crate::core::metrics::inc_http_requests;

const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// HTTP response compression middleware (brotli, zstd, gzip, deflate)
/// with 512-byte minimum payload threshold.
pub fn compression_middleware() -> Compression {
    Compression::new()
        .min_length(512)
        .force_priority(true)
}

/// Request timeout & HTTP metrics counter middleware.
/// Skips long-lived streaming endpoints (SSE `/notifications/stream` & WebSocket `/notifications/ws`).
#[handler]
pub async fn request_timeout_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    inc_http_requests();

    let path = req.uri().path();
    if path.ends_with("/notifications/stream") || path.ends_with("/notifications/ws") {
        ctrl.call_next(req, depot, res).await;
        return;
    }

    let timeout_dur = Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_SECS);
    if tokio::time::timeout(timeout_dur, ctrl.call_next(req, depot, res))
        .await
        .is_err()
    {
        ctrl.skip_rest();
        res.status_code(StatusCode::SERVICE_UNAVAILABLE);
        res.render(Json(serde_json::json!({
            "error": {
                "code": "REQUEST_TIMEOUT",
                "message": "Waktu pemrosesan permintaan habis (timeout)"
            }
        })));
    }
}
