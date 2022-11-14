use crate::SecretPicker;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub channel_id: String,
    pub thread_ts: String,
    pub puzzle_number: i64,
    pub date: String,
    pub active: bool,
    pub secret: String,
}

impl Game {
    pub async fn get(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Game>> {
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM game
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(game) = result {
            Ok(Some(Game {
                id: game.id,
                channel_id: game.channel_id,
                thread_ts: game.thread_ts,
                puzzle_number: game.puzzle_number,
                date: game.date,
                active: game.active,
                secret: game.secret,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn new(
        pool: &SqlitePool,
        channel_id: String,
        thread_ts: String,
        puzzle_number: i64,
        date: String,
        active: bool,
    ) -> anyhow::Result<Game> {
        let picker = SecretPicker::new(channel_id.as_str());
        let secret = picker.get_secret(puzzle_number as u32).to_string();

        let result = sqlx::query!(
            r#"
            INSERT INTO game (channel_id, thread_ts, puzzle_number, date, active, secret)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            channel_id,
            thread_ts,
            puzzle_number,
            date,
            active,
            secret,
        )
        .execute(pool)
        .await?
        .last_insert_rowid();

        Ok(Game {
            id: result,
            channel_id,
            thread_ts,
            puzzle_number,
            date,
            active,
            secret,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn test_creating_game(pool: SqlitePool) -> anyhow::Result<()> {
        let channel_id = String::from("channel_x");

        // Insert a channel into the channel table for a foreign key constraint
        sqlx::query!(
            r#"
            INSERT INTO channel (id, team_id, hour, active)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            channel_id,
            "team_id",
            8,
            true,
        )
        .execute(&pool)
        .await?;

        let thread_ts = String::from("1234.567");
        let puzzle_number = 123;
        let date = String::from("Tuesday September 6");
        let active = true;

        let game = Game::new(
            &pool,
            channel_id.clone(),
            thread_ts.clone(),
            puzzle_number,
            date.clone(),
            active,
        )
        .await?;

        // Expect game to have an id
        assert_eq!(game.id, 1);
        // Expected secret looked up with SecretPicker separately
        assert_eq!(game.secret, "among");

        // Create a following game
        //
        let game2 = Game::new(
            &pool,
            channel_id,
            thread_ts,
            puzzle_number + 1,
            date,
            active,
        )
        .await?;

        // Next game increments the id
        assert_eq!(game2.id, 2);
        // Expected secret looked up with SecretPicker separately
        assert_eq!(game2.secret, "serving");

        Ok(())
    }

    #[sqlx::test]
    async fn test_getting_game(pool: SqlitePool) -> anyhow::Result<()> {
        // Pre-insert channel and a game to then fetch
        let channel_id = String::from("channel_x");

        // Insert a channel into the channel table for a foreign key constraint
        sqlx::query!(
            r#"
            INSERT INTO channel (id, team_id, hour, active)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            channel_id,
            "team_id",
            8,
            true,
        )
        .execute(&pool)
        .await?;

        let game_id = sqlx::query!(
            r#"
            INSERT INTO game (channel_id, thread_ts, puzzle_number, date, active, secret)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            channel_id,
            "",
            123,
            "",
            true,
            "among",
        )
        .execute(&pool)
        .await?
        .last_insert_rowid();

        let game = Game::get(&pool, game_id).await?;

        assert!(game.is_some());

        let game = game.unwrap();

        assert_eq!(game.id, game_id);

        Ok(())
    }
}
