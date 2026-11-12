use crate::modules::user::handler::*;
use salvo::Router;

pub fn user_routes() -> Router {

    Router::with_path("users")
        .get(get_all_user)
        .post(create_user)
        .push(
            Router::with_path("{user_id}")
                .get(get_user_by_id)
                .patch(edit_user)
                .delete(delete_user)
                .push(Router::with_path("tasks").get(get_user_tasks))
                .push(Router::with_path("task-comments").get(get_user_task_comments))
                .push(Router::with_path("task-sharings").get(get_user_task_comments))
        )


}