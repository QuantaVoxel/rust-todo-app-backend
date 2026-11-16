use salvo::cors::{AllowOrigin, Cors, CorsHandler};
use salvo::http::Method;
use salvo::prelude::*;

pub fn cors_middleware() -> CorsHandler {
    Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            "authorization",
            "content-type",
            "x-requested-with",
            "accept",
            "origin",
        ])
        .expose_headers(vec![
            "x-request-id",
            "x-ratelimit-limit",
            "x-ratelimit-remaining",
            "x-ratelimit-reset",
        ])
        .max_age(3600)
        .into_handler()
}

#[handler]
pub async fn security_headers_middleware(res: &mut Response) {
    let h = res.headers_mut();
    if let Ok(val) = "nosniff".parse() {
        h.insert("x-content-type-options", val);
    }
    if let Ok(val) = "DENY".parse() {
        h.insert("x-frame-options", val);
    }
    if let Ok(val) = "strict-origin-when-cross-origin".parse() {
        h.insert("referrer-policy", val);
    }
    if let Ok(val) = "1; mode=block".parse() {
        h.insert("x-xss-protection", val);
    }
}
