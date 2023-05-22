#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserInfoDbModel {
    pub user_id: i64,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub hashed_password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserInfoModel {
    User {
        email: String,
        token: String,
        username: String,
        bio: Option<String>,
        image: Option<String>,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserInfoForLogin {
    User { email: String, password: String },
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserInfoForRegistration {
    User {
        username: String,
        email: String,
        password: String,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserInfoForUpdate {
    User {
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        image: Option<String>,
        bio: Option<String>,
    },
}
