use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::core::middleware::get_authenticated_user;

#[handler]
pub async fn list_activity_logs(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}
