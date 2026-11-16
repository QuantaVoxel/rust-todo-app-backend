use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::connections::dtos::*;
use crate::modules::connections::services::ConnectionService;

#[handler]
pub async fn list_connections(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let status_filter = req.query::<String>("status");

    match ConnectionService::list_connections(user.user_id, status_filter).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn create_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let dto = match parse_and_validate::<CreateConnectionRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ConnectionService::create_connection(user.user_id, dto).await {
        Ok(data) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(ApiResponse::new(data)));
        }
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let connection_id = req.param::<i32>("connection_id").unwrap_or_default();
    let dto = match parse_and_validate::<UpdateConnectionRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match ConnectionService::update_connection(user.user_id, connection_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_connection(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let connection_id = req.param::<i32>("connection_id").unwrap_or_default();

    match ConnectionService::delete_connection(user.user_id, connection_id).await {
        Ok(_) => res.render(Json(ApiResponse::new("Koneksi berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}
