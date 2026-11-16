use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::connections::dtos::*;

#[handler]
pub async fn list_connections(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
}

#[handler]
pub async fn create_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _dto = match parse_and_validate::<CreateConnectionRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Connection requested")));
}

#[handler]
pub async fn update_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _connection_id = req.param::<i32>("connection_id").unwrap_or_default();
    let _dto = match parse_and_validate::<UpdateConnectionRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    res.render(Json(ApiResponse::new("Connection updated")));
}

#[handler]
pub async fn delete_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _connection_id = req.param::<i32>("connection_id").unwrap_or_default();
    res.render(Json(ApiResponse::new("Connection deleted")));
}
