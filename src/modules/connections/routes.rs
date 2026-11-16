use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::connections::handlers;

pub fn routes() -> Router {
    Router::with_path("connections")
        .hoop(auth_middleware)
        .get(handlers::list_connections)
        .post(handlers::create_connection)
        .push(
            Router::with_path("{connection_id}")
                .patch(handlers::update_connection)
                .delete(handlers::delete_connection),
        )
}
