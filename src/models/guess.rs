use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Guess {
    pub id: Uuid,
    pub game_id: Uuid,
    pub updated: i64,
    pub user_id: String,
    pub word: String,
    pub rank: i64,
    pub similarity: f64,
    pub guess_num: Option<i64>,
    pub latest_guess_user_id: String,
}

impl Guess {
    pub async fn get(
        game_id: Uuid,
        word: &str,
        db: &sqlx::PgPool,
    ) -> Result<Option<Guess>, sqlx::Error> {
        let guess = sqlx::query_as!(
            Guess,
            r#"
            SELECT 
                *
            FROM
                guess
            WHERE
                game_id = $1 AND
                word = $2
            "#,
            game_id,
            word
        )
        .fetch_optional(db)
        .await?;
        Ok(guess)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;
        sqlx::query!(
            r#"
            INSERT INTO 
                guess(
                    id,
                    game_id,
                    updated,
                    user_id,
                    word,
                    rank,
                    similarity,
                    latest_guess_user_id
                )
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8);
            "#,
            self.id,
            self.game_id,
            self.updated,
            self.user_id,
            self.word,
            self.rank,
            self.similarity,
            self.latest_guess_user_id,
        )
        .execute(&mut *tx)
        .await?;

        // Update the guess_num to be the latest guess
        sqlx::query!(
            r#"
            UPDATE
                guess
            SET
                guess_num = s.row_num
            FROM (
                SELECT 
                    id,
                    guess_num, 
                    ROW_NUMBER() OVER (PARTITION BY game_id ORDER BY guess_num) AS row_num
                FROM guess
            ) AS s
            WHERE guess.id = s.id; 
            "#,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn update_latest_guess_user_id(
        &mut self,
        user_id: &str,
        db: &sqlx::PgPool,
    ) -> Result<(), sqlx::Error> {
        self.updated = chrono::Utc::now().timestamp_millis();
        self.latest_guess_user_id = user_id.to_string();

        sqlx::query!(
            r#"
            UPDATE
                guess
            SET
                updated = $1,
                latest_guess_user_id = $2
            WHERE
                id = $3
            "#,
            self.updated,
            self.latest_guess_user_id,
            self.id,
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub fn is_secret(&self) -> bool {
        self.rank == 0
    }
}
