use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Game {
    pub id: Uuid,
    pub channel_id: String,
    pub thread_ts: Option<String>,
    pub puzzle_number: i64,
    pub date: DateTime<Utc>,
    pub active: bool,
    pub secret: String,
    pub hint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GuessContext {
    pub guess_num: i64,
    pub word: String,
    pub username: String,
    pub profile_photo: String,
    pub rank: i64,
    pub similarity: f64,
}

pub enum GuessContextOrder {
    Rank,
    GuessUpdated,
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

    pub async fn get_by_id(id: Uuid, db: &sqlx::PgPool) -> Result<Option<Game>, sqlx::Error> {
        log::debug!("Fetching game for id: {}", id);
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT 
                *
            FROM
                game
            WHERE
                id = $1
            "#,
            id
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

    pub async fn get_guess_count(&self, db: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            r#"
            SELECT
                count(*)
            FROM
                guess
            WHERE
                game_id = $1
            "#,
            self.id,
        )
        .fetch_one(db)
        .await?
        .count
        .map_or(Ok(0), Ok)
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
                guess_num,
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
                GuessContextOrder::GuessUpdated => "updated DESC",
            },
        );

        let guesses: Vec<GuessContext> = sqlx::query_as(q.as_str())
            .bind(self.id)
            .bind(count)
            .fetch_all(db)
            .await?;

        Ok(guesses)
    }

    pub async fn get_next_puzzle_number(channel_id: String, db: &sqlx::PgPool) -> i64 {
        let last_puzzle_number = match sqlx::query!(
            r#"
            SELECT
                puzzle_number
            FROM
                game
            WHERE
                channel_id = $1
            ORDER BY
                puzzle_number DESC
            LIMIT 1
            "#,
            channel_id
        )
        .fetch_optional(db)
        .await
        {
            Ok(Some(game)) => game.puzzle_number,
            _ => 0,
        };

        last_puzzle_number + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::channel::Channel;

    #[sqlx::test]
    async fn test_get_next_puzzle_number_with_no_games_is_1(
        pool: sqlx::PgPool,
    ) -> sqlx::Result<()> {
        let channel_id = "channel_id".to_string();

        let next_puzzle_number = Game::get_next_puzzle_number(channel_id, &pool).await;

        assert_eq!(next_puzzle_number, 1);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_next_puzzle_number_increments_last_highest(
        pool: sqlx::PgPool,
    ) -> sqlx::Result<()> {
        // Insert a channel for the game
        let channel_id = "channel_id".to_string();
        let channel = Channel {
            id: channel_id.clone(),
            team_id: "team_id".to_string(),
            hour: 0,
            active: true,
        };
        channel.insert(&pool).await?;

        let game_id = Uuid::new_v4();
        let game = Game {
            id: game_id,
            channel_id: channel_id.clone(),
            thread_ts: None,
            puzzle_number: 3,
            date: datetime!(2022, 5, 7),
            active: true,
            hint: None,
            secret: "secret".to_string(),
        };
        game.insert(&pool).await?;

        let next_puzzle_number = Game::get_next_puzzle_number(channel_id, &pool).await;

        assert_eq!(next_puzzle_number, 4);

        Ok(())
    }
}
