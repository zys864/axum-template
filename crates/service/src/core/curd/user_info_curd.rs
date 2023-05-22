use crate::{model::user_info::UserInfoDbModel, DbPool};

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
    pub async fn update_user(db: &DbPool, user: &UserInfoDbModel) -> sqlx::Result<i64> {
        let updated_user = sqlx::query!(
            r#"
            UPDATE  user_info SET email = $2, username=$3, hashed_password=$4, bio=$5, image=$6
            WHERE user_id = $1
            RETURNING user_id
            "#,
            user.user_id,
            user.email,
            user.username,
            user.hashed_password,
            user.bio,
            user.image
        )
        .fetch_one(db)
        .await?;
        Ok(updated_user.user_id)
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
