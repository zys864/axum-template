use crate::{
    http::{helper::auth::AuthUtils, schema::user_info::UserInfoRspModel},
    model::user_info::{UserInfoDbModel, UserInfoDbUpdateModel},
    DbPool,
};

pub struct UserInfoCurd;

impl UserInfoCurd {
    pub async fn get_by_email(db: &DbPool, email: &str) -> sqlx::Result<Option<UserInfoDbModel>> {
        let user = sqlx::query_as!(
            UserInfoDbModel,
            "SELECT * FROM user_info WHERE email=$1",
            email
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }
    pub async fn is_duplicate_email(db: &DbPool, email: &str) -> sqlx::Result<bool> {
        let is_duplicate = sqlx::query!("SELECT email FROM user_info WHERE email=$1", email)
            .fetch_optional(db)
            .await?
            .map(|_| true)
            .unwrap_or(false);
        Ok(is_duplicate)
    }
    pub async fn create(
        db: &DbPool,
        username: &str,
        email: &str,
        hashed_password: &str,
    ) -> sqlx::Result<()> {
        let _res = sqlx::query!(
            "INSERT INTO user_info (username,email,hashed_password) VALUES ($1,$2,$3)",
            username,
            email,
            hashed_password,
        )
        .execute(db)
        .await?;
        Ok(())
    }
    pub async fn get_by_username(
        db: &DbPool,
        username: &str,
    ) -> sqlx::Result<Option<UserInfoDbModel>> {
        let user = sqlx::query_as!(
            UserInfoDbModel,
            "SELECT * FROM user_info WHERE username=$1",
            username
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }
    pub async fn update_user(
        db: &DbPool,
        email:&str,
        user_for_update: &UserInfoDbUpdateModel,
    ) -> crate::error::HttpResult<UserInfoRspModel> {
    
        let hased_passwd = if let Some(passwd) = user_for_update.password.as_ref() {
            Some(AuthUtils::hash(&passwd)?)
        } else {
            None
        };
        let updated_user = sqlx::query!(
            r#"
            UPDATE "user_info" 
                SET email = COALESCE ( $1, user_info.email ),
                username = COALESCE ( $2, user_info.username ),
                hashed_password = COALESCE ( $3, user_info.hashed_password ),
                bio = COALESCE ( $4, user_info.bio ),
                image = COALESCE ( $5, user_info.image ) 
            WHERE
                email =$6
            RETURNING email, username, bio, image
            "#,
            user_for_update.email,
            user_for_update.username,
            hased_passwd,
            user_for_update.bio,
            user_for_update.image,
            email
        )
        .fetch_one(db)
        .await?;
        Ok(UserInfoRspModel::new(
            updated_user.email,
            updated_user.username,
            updated_user.bio,
            updated_user.image,
        )?)
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::test_db::TestDb;

    use super::*;
    fn user_info_csv() -> String {
        "email,username,bio,image,hashed_password\nemail@email.com,sys_user,,,11111".to_string()
    }
    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_name() {
        let email = "email@email.com".to_string();
        let test_db = TestDb::from_env();
        test_db
            .load_csv_data("user_info", &user_info_csv())
            .await
            .unwrap();
        let pool = test_db.get_pool().await;

        let res = UserInfoCurd::get_by_email(&pool, &email).await;
        tracing::info!("msg = {:?}", res);
    }
}
