use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::users::handlers;

pub fn routes() -> Router {
    Router::with_path("users")
        .hoop(auth_middleware)
        .push(Router::new().get(handlers::search_users))
        .push(
            Router::with_path("me")
                .get(handlers::get_me)
                .patch(handlers::update_me)
                .delete(handlers::delete_me),
        )
        .push(
            Router::with_path("me/profile")
                .get(handlers::get_profile)
                .put(handlers::upsert_profile),
        )
        .push(
            Router::with_path("me/settings")
                .get(handlers::get_settings)
                .push(
                    Router::with_path("{setting_key}")
                        .put(handlers::upsert_setting)
                        .delete(handlers::delete_setting),
                ),
        )
}
