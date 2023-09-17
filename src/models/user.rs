use crate::SimilariumError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub profile_photo: String,
    pub username: String,
}

impl User {
    pub async fn get(user_id: &str, db: &sqlx::PgPool) -> Result<Option<User>, SimilariumError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                *
            FROM
                "user"
            WHERE
                id = $1
            "#,
            user_id
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), SimilariumError> {
        sqlx::query!(
            r#"
            INSERT INTO 
                "user"(id, profile_photo, username)
            VALUES ($1, $2, $3);
            "#,
            self.id,
            self.profile_photo,
            self.username,
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
