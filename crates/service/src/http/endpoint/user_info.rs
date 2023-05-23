use axum::{extract::State, Json};

use crate::{
    http::{helper::{jwt::Claims, auth::verify_hash}, state::AppState},
    model::user_info::{UserInfoDbModel, UserInfoForLogin},
};

pub struct UserInfoService;

impl UserInfoService {
    /// # Authentication
    ///
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#authentication
    pub async fn login(
        State(state): State<AppState>,
        Json(user_login):Json<UserInfoForLogin>
    ) -> Json<Vec<UserInfoDbModel>> {
        // let user_info =
        let hashed_passwd = verify_hash("","");
        let users = sqlx::query_as!(UserInfoDbModel, "SELECT * FROM user_info")
            .fetch_all(&state.db_session)
            .await
            .unwrap();
        Json(users)
    }
}
