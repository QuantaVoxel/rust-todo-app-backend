use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::error::ApiError;
use crate::modules::users::dtos::*;
use crate::modules::users::services::UserService;

#[handler]
pub async fn get_me(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::get_me(user.user_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn update_me(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let dto = match parse_and_validate::<UpdateUserRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::update_me(user.user_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_me(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::delete_me(user.user_id, Some(&user.token_jti)).await {
        Ok(_) => res.render(Json(ApiResponse::new("Akun berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn search_users(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let search = req.query::<String>("search").unwrap_or_default();

    match UserService::search_users(user.user_id, &search).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_profile(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::get_profile(user.user_id).await {
        Ok(profile) => res.render(Json(ApiResponse::new(profile))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upsert_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let dto = match parse_and_validate::<UpsertProfileRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::upsert_profile(user.user_id, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn get_settings(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::get_settings(user.user_id).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn upsert_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let key = match req.param::<String>("setting_key") {
        Some(k) if !k.trim().is_empty() => k,
        _ => {
            res.render(ApiError::BadRequest("setting_key tidak valid".into()));
            return;
        }
    };

    let dto = match parse_and_validate::<UpsertSettingRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    match UserService::upsert_setting(user.user_id, key, dto).await {
        Ok(data) => res.render(Json(ApiResponse::new(data))),
        Err(err) => res.render(err),
    }
}

#[handler]
pub async fn delete_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let key = match req.param::<String>("setting_key") {
        Some(k) if !k.trim().is_empty() => k,
        _ => {
            res.render(ApiError::BadRequest("setting_key tidak valid".into()));
            return;
        }
    };

    match UserService::delete_setting(user.user_id, &key).await {
        Ok(_) => res.render(Json(ApiResponse::new("Pengaturan berhasil dihapus"))),
        Err(err) => res.render(err),
    }
}
