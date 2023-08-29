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

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GuessContext {
    pub word: String,
    pub username: String,
    pub profile_photo: String,
    pub rank: i64,
    pub similarity: f64,
}

pub enum GuessContextOrder {
    Rank,
    GuessNum,
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

    pub async fn get_guess_contexts(
        &self,
        order: GuessContextOrder,
        count: i64,
        db: &sqlx::PgPool,
    ) -> Result<Vec<GuessContext>, sqlx::Error> {
        let q = format!(
            r#"
            SELECT
                word,
                username,
                profile_photo,
                rank,
                similarity
            FROM
                guess g
            LEFT JOIN
                "user" u
            ON
                g.user_id = u.id
            WHERE
                game_id = $1
            ORDER BY
                {}
            LIMIT $2
            "#,
            match order {
                GuessContextOrder::Rank => "rank ASC",
                GuessContextOrder::GuessNum => "guess_num DESC",
            },
        );

        let guesses: Vec<GuessContext> = sqlx::query_as(q.as_str())
            .bind(self.id)
            .bind(count)
            .fetch_all(db)
            .await?;

        Ok(guesses)
    }
}
