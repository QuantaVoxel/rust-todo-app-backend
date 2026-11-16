use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::modules::auth::dtos::*;

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<RegisterRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    // To be wired to auth service
    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new("Registered placeholder")));
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<LoginRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    // To be wired to auth service
    res.render(Json(ApiResponse::new("Login placeholder")));
}

#[handler]
pub async fn refresh_token(req: &mut Request, res: &mut Response) {
    let _dto = match parse_and_validate::<RefreshTokenRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Refresh placeholder")));
}

#[handler]
pub async fn logout(_req: &mut Request, res: &mut Response) {
    res.render(Json(ApiResponse::new("Logout placeholder")));
}

#[handler]
pub async fn change_password(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match crate::core::middleware::get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let _dto = match parse_and_validate::<ChangePasswordRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Change password placeholder")));
}
