use axum::{
    routing::{post},
    Router,
};

use self::user_info::UserInfoService;

use super::state::AppState;

pub mod user_info;

pub fn user_info_router() -> Router<AppState> {
    Router::new()
        .route(
            "/user",
            post(UserInfoService::login).get(UserInfoService::get_current_user),
        )
        .route("/users", post(UserInfoService::user_registration))
}
