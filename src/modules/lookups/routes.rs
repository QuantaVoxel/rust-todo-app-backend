use salvo::Router;

use crate::core::cache::{etag_caching_headers, lookup_response_cache};
use crate::modules::lookups::handlers;

pub fn routes() -> Router {
    Router::new()
        .hoop(etag_caching_headers())
        .hoop(lookup_response_cache())
        .push(Router::with_path("statuses").get(handlers::get_statuses))
        .push(Router::with_path("priorities").get(handlers::get_priorities))
        .push(Router::with_path("external-apps").get(handlers::get_external_apps))
        .push(Router::with_path("enums").get(handlers::get_enums))
}
