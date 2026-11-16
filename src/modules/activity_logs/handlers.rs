use salvo::prelude::*;

use crate::core::middleware::get_authenticated_user;
use crate::modules::activity_logs::dtos::ActivityLogQuery;
use crate::modules::activity_logs::services::ActivityLogService;

#[handler]
pub async fn list_activity_logs(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let query = req.parse_queries::<ActivityLogQuery>().unwrap_or(ActivityLogQuery {
        entity_type: None,
        entity_id: None,
        action: None,
        from: None,
        to: None,
        page: None,
        per_page: None,
    });

    match ActivityLogService::list_activity_logs(user.user_id, query).await {
        Ok(paginated) => res.render(Json(paginated)),
        Err(err) => res.render(err),
    }
}
