use axum::{extract::State, Json};
use tracing::instrument;

use crate::{
    core::curd::user_info_curd::UserInfoCurd,
    error::HttpResult,
    http::{
        helper::{auth::AuthUtils, jwt::Claims},
        schema::user_info::{
            Taggeduser, UserInfoForLogin, UserInfoForRegistration, UserInfoRspModel,
        },
        state::AppState,
    },
    model::user_info::UserInfoDbUpdateModel,
};

pub struct UserInfoService;

impl UserInfoService {
    /// # Authentication
    ///
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#authentication
    #[instrument(skip_all, err(Debug))]
    pub async fn login(
        State(state): State<AppState>,
        Json(Taggeduser { user }): Json<Taggeduser<UserInfoForLogin>>,
    ) -> HttpResult<Json<Taggeduser<UserInfoRspModel>>> {
        let user_info = UserInfoCurd::get_by_email(&state.db_session, &user.email)
            .await?
            .ok_or(crate::error::ErrorKind::NoSuchUserOrErrorPassword)?;
        let is_correct_passwd = AuthUtils::verify_hash(&user.password, &user_info.hashed_password)?;
        if is_correct_passwd {
            Ok(Json(Taggeduser {
                user: UserInfoRspModel::try_from(user_info)?,
            }))
        } else {
            Err(crate::error::ErrorKind::NoSuchUserOrErrorPassword)
        }
    }

    /// # Registration
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints/#registration
    #[instrument(skip_all, err(Debug))]
    pub async fn user_registration(
        State(state): State<AppState>,
        Json(Taggeduser { user }): Json<Taggeduser<UserInfoForRegistration>>,
    ) -> HttpResult<Json<Taggeduser<UserInfoRspModel>>> {
        if UserInfoCurd::is_duplicate_email(&state.db_session, &user.email).await? {
            return Err(crate::error::ErrorKind::DuplicatedEmail(user.email));
        }
        let hashed_password = AuthUtils::hash(&user.password)?;
        UserInfoCurd::create(
            &state.db_session,
            &user.username,
            &user.email,
            &hashed_password,
        )
        .await?;
        let user_info_rsp_model = UserInfoRspModel {
            email: user.username,
            username: user.email.clone(),
            bio: None,
            image: None,
            token: Claims::new(user.email).generate_jwt_token()?,
        };
        Ok(Json(Taggeduser {
            user: user_info_rsp_model,
        }))
    }

    /// # Get Current User
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints/#get-current-user
    #[instrument(skip_all, err(Debug))]
    pub async fn get_current_user(
        claims: Claims,
        state: State<AppState>,
    ) -> HttpResult<Json<Taggeduser<UserInfoRspModel>>> {
        let user = UserInfoCurd::get_by_email(&state.db_session, &claims.email)
            .await?
            .ok_or(crate::error::ErrorKind::NoSuchUserOrErrorPassword)?;
        let user_info_rsp_model = UserInfoRspModel::try_from(user)?;
        Ok(Json(Taggeduser {
            user: user_info_rsp_model,
        }))
    }

    /// # Get Current User
    /// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints/#get-current-user
    #[instrument(skip_all, err(Debug))]
    pub async fn update_user_info(
        claims: Claims,
        state: State<AppState>,
        Json(Taggeduser { user }): Json<Taggeduser<UserInfoDbUpdateModel>>,
    ) -> HttpResult<Json<Taggeduser<UserInfoRspModel>>> {
        if user == UserInfoDbUpdateModel::default() {
            return Self::get_current_user(claims, state).await;
        }
        if user.email.is_some()
            && UserInfoCurd::is_duplicate_email(&state.db_session, &claims.email).await?
        {
            return Err(crate::error::ErrorKind::DuplicatedEmail(claims.email));
        }
        let user = UserInfoCurd::update_user(&state.db_session, &claims.email, &user).await?;
        Ok(Json(Taggeduser { user }))
    }
}
