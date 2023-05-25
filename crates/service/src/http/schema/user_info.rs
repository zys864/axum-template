use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{http::helper::jwt, model::user_info::UserInfoDbModel};
#[derive(Debug, Serialize, Deserialize)]
pub struct Taggeduser<T> {
    pub user: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoRspModel {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoForLogin {
    pub email: String,
    pub password: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoForRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoForUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl UserInfoRspModel {
    pub fn new(
        email: String,
        username: String,
        bio: Option<String>,
        image: Option<String>,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = jwt::Claims::new(email.clone()).generate_jwt_token()?;

        Ok(Self {
            email,
            token,
            username,
            bio,
            image,
        })
    }
}
impl TryFrom<UserInfoDbModel> for UserInfoRspModel {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(user: UserInfoDbModel) -> Result<Self, Self::Error> {
        let token = jwt::Claims::new(user.email.clone()).generate_jwt_token()?;
        Ok(Self {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        })
    }
}
