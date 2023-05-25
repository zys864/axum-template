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
pub struct UserInfoDbCreateModel {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub hashed_password: String,
}
