use axum::{routing::get, Router};

use self::user_info::UserInfoService;

use super::state::AppState;

pub mod user_info;

pub fn user_info_router() -> Router<AppState> {
    Router::new().route("/user", get(UserInfoService::login))
}
