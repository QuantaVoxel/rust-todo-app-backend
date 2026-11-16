use salvo::Router;

use crate::core::middleware::auth_middleware;
use crate::modules::tasks::handlers;

pub fn routes() -> Router {
    Router::with_path("tasks")
        .hoop(auth_middleware)
        .get(handlers::list_tasks)
        .post(handlers::create_task)
        .push(
            Router::with_path("{task_id}")
                .get(handlers::get_task)
                .patch(handlers::update_task)
                .delete(handlers::delete_task)
                .push(Router::with_path("archive").post(handlers::archive_task))
                .push(Router::with_path("unarchive").post(handlers::unarchive_task))
                .push(Router::with_path("progress").put(handlers::update_progress)),
        )
}
