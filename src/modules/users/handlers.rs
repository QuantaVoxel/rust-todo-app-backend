use salvo::prelude::*;

use crate::common::response::ApiResponse;
use crate::common::validation::parse_and_validate;
use crate::core::middleware::get_authenticated_user;
use crate::modules::users::dtos::*;

#[handler]
pub async fn get_me(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new(format!("User info for ID {}", user.user_id))));
}

#[handler]
pub async fn update_me(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let _dto = match parse_and_validate::<UpdateUserRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("User updated")));
}

#[handler]
pub async fn delete_me(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("User deleted")));
}

#[handler]
pub async fn search_users(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let _search = req.query::<String>("search").unwrap_or_default();
    res.render(Json(ApiResponse::new(Vec::<String>::new())));
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

    res.render(Json(ApiResponse::new(format!("Profile for user {}", user.user_id))));
}

#[handler]
pub async fn upsert_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    let _dto = match parse_and_validate::<UpsertProfileRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Profile updated")));
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

    res.render(Json(ApiResponse::new(format!("Settings for user {}", user.user_id))));
}

#[handler]
pub async fn upsert_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _key = req.param::<String>("setting_key").unwrap_or_default();

    let _dto = match parse_and_validate::<UpsertSettingRequest>(req).await {
        Ok(d) => d,
        Err(err) => {
            res.render(err);
            return;
        }
    };

    res.render(Json(ApiResponse::new("Setting saved")));
}

#[handler]
pub async fn delete_setting(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let _user = match get_authenticated_user(depot) {
        Ok(u) => u,
        Err(err) => {
            res.render(err);
            return;
        }
    };
    let _key = req.param::<String>("setting_key").unwrap_or_default();

    res.render(Json(ApiResponse::new("Setting deleted")));
}
