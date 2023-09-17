use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct GameWinnerAssociation {
    pub game_id: Uuid,
    pub user_id: String,
    pub guess_idx: i64,
    pub created: i64,
}

impl GameWinnerAssociation {
    pub async fn get(
        game_id: Uuid,
        user_id: &str,
        db: &sqlx::PgPool,
    ) -> Result<Option<GameWinnerAssociation>, sqlx::Error> {
        let association = sqlx::query_as!(
            GameWinnerAssociation,
            r#"
            SELECT
                *
            FROM
                game_user_winner_association
            WHERE
                game_id = $1 AND
                user_id = $2
            "#,
            game_id,
            user_id
        )
        .fetch_optional(db)
        .await?;

        Ok(association)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO
                game_user_winner_association(game_id, user_id, guess_idx, created)
            VALUES
                ($1, $2, $3, $4);
            "#,
            self.game_id,
            self.user_id,
            self.guess_idx,
            self.created,
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
