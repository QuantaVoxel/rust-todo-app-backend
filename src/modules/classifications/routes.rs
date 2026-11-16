use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::classifications::handlers;

pub fn routes() -> Router {
    Router::new()
        .hoop(auth_middleware)
        .push(
            Router::with_path("tags")
                .get(handlers::list_tags)
                .post(handlers::create_tag),
        )
        .push(
            Router::with_path("categories")
                .get(handlers::list_categories)
                .post(handlers::create_category),
        )
        .push(
            Router::with_path("labels")
                .get(handlers::list_labels)
                .post(handlers::create_label),
        )
}
