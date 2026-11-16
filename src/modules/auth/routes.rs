use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::auth::handlers;

pub fn routes() -> Router {
    let public_routes = Router::with_path("auth")
        .push(Router::with_path("register").post(handlers::register))
        .push(Router::with_path("login").post(handlers::login))
        .push(Router::with_path("refresh").post(handlers::refresh_token))
        .push(Router::with_path("logout").post(handlers::logout).delete(handlers::logout));

    let protected_routes = Router::with_path("auth")
        .hoop(auth_middleware)
        .push(Router::with_path("change-password").post(handlers::change_password));

    Router::new().push(public_routes).push(protected_routes)
}
