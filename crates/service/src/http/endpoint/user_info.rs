use axum::{extract::State, Json};

use crate::{http::state::AppState, model::user_info::UserInfoDbModel};

pub struct UserInfoService;

impl UserInfoService {
    pub async fn user_info(State(state): State<AppState>) -> Json<Vec<UserInfoDbModel>> {
        let users = sqlx::query_as!(UserInfoDbModel, "SELECT * FROM user_info")
            .fetch_all(&state.db_session)
            .await
            .unwrap();
        Json(users)
    }
}
