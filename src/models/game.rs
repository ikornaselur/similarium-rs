use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Game {
    pub id: Uuid,
    pub channel_id: String,
    pub thread_ts: Option<String>,
    pub puzzle_number: i64,
    pub date: String,
    pub active: bool,
    pub secret: String,
    pub hint: Option<String>,
}

impl Game {
    pub async fn get(
        channel_id: &str,
        thread_ts: &str,
        db: &sqlx::PgPool,
    ) -> Result<Option<Game>, sqlx::Error> {
        log::debug!(
            "Fetching game for channel: {} and thread_ts: {}",
            channel_id,
            thread_ts
        );
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT 
                *
            FROM
                game
            WHERE
                channel_id = $1 AND
                thread_ts = $2
            "#,
            channel_id,
            thread_ts
        )
        .fetch_optional(db)
        .await?;
        Ok(game)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO 
                game(
                    id,
                    channel_id,
                    thread_ts,
                    puzzle_number,
                    date,
                    active,
                    secret,
                    hint
                )
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8);
            "#,
            self.id,
            self.channel_id,
            self.thread_ts,
            self.puzzle_number,
            self.date,
            self.active,
            self.secret,
            self.hint,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn set_thread_ts(
        &mut self,
        thread_ts: &str,
        db: &sqlx::PgPool,
    ) -> Result<(), sqlx::Error> {
        log::debug!("Setting thread_ts to {}", thread_ts);
        self.thread_ts = Some(thread_ts.to_string());
        sqlx::query!(
            r#"
            UPDATE
                game
            SET
                thread_ts = $1
            WHERE
                id = $2
            "#,
            self.thread_ts,
            self.id,
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
