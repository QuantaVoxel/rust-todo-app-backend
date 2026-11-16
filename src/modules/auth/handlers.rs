use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::auth::dtos::*;
use crate::modules::auth::services::AuthService;

#[endpoint(tags("Auth"), summary = "Register new user")]
pub async fn register(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<RegisterRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match AuthService::register(dto).await {
        Ok(result) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(result));
        }
        Err(err) => {
            res.render(err);
        }
    }
}

#[endpoint(tags("Auth"), summary = "Login user and get tokens")]
pub async fn login(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<LoginRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match AuthService::login(dto).await {
        Ok(result) => {
            res.render(Json(result));
        }
        Err(err) => {
            res.render(err);
        }
    }
}

#[endpoint(tags("Auth"), summary = "Refresh access and refresh tokens")]
pub async fn refresh_token(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<RefreshTokenRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match AuthService::refresh_token(dto).await {
        Ok(result) => {
            res.render(Json(result));
        }
        Err(err) => {
            res.render(err);
        }
    }
}

#[endpoint(tags("Auth"), summary = "Logout and invalidate tokens", security(("bearer_auth" = [])))]
pub async fn logout(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let logout_dto = parse_and_validate::<LogoutRequest>(req).await.ok();
    let auth_jti = get_authenticated_user(depot).ok().map(|u| u.token_jti.as_str());

    match AuthService::logout(logout_dto, auth_jti).await {
        Ok(_) => {
            res.render(Json(ApiResponse::new("Logout berhasil")));
        }
        Err(err) => {
            res.render(err);
        }
    }
}

#[endpoint(tags("Auth"), summary = "Change password for authenticated user", security(("bearer_auth" = [])))]
pub async fn change_password(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let auth_user = match get_authenticated_user(depot) {
        Ok(u) => u.clone(),
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let dto = match parse_and_validate::<ChangePasswordRequest>(req).await {
        Ok(dto) => dto,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match AuthService::change_password(auth_user.user_id, dto).await {
        Ok(_) => {
            res.render(Json(ApiResponse::new("Kata sandi berhasil diubah")));
        }
        Err(err) => {
            res.render(err);
        }
    }
}
