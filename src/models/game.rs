use crate::models::GameWinnerAssociation;
use crate::SimilariumError;
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
    pub taunt_index: i64,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct GuessContext {
    pub guess_num: i64,
    pub word: String,
    pub username: String,
    pub profile_photo: String,
    pub rank: i64,
    pub similarity: f64,
    pub is_secret: bool,
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
    ) -> Result<Option<Game>, SimilariumError> {
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

    pub async fn get_by_id(id: Uuid, db: &sqlx::PgPool) -> Result<Option<Game>, SimilariumError> {
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

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), SimilariumError> {
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
                    hint,
                    taunt_index
                )
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9);
            "#,
            self.id,
            self.channel_id,
            self.thread_ts,
            self.puzzle_number,
            self.date,
            self.active,
            self.secret,
            self.hint,
            self.taunt_index,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn set_active(
        &mut self,
        active: bool,
        db: &sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        log::debug!("[Game: {}] Setting active to {}", self.id, active);
        self.active = active;
        sqlx::query!(
            r#"
            UPDATE 
                game
            SET 
                active = $1
            WHERE
                id = $2
            "#,
            self.active,
            self.id,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn set_thread_ts(
        &mut self,
        thread_ts: &str,
        db: &sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        log::debug!("[Game: {}] Setting thread_ts to {}", self.id, thread_ts);
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

    pub async fn set_taunt_index(
        &mut self,
        taunt_index: i64,
        db: &sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        log::debug!("[Game: {}] Setting taunt_index to {}", self.id, taunt_index);
        self.taunt_index = taunt_index;
        sqlx::query!(
            r#"
            UPDATE
                game
            SET
                taunt_index = $1
            WHERE
                id = $2
            "#,
            self.taunt_index,
            self.id,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_guess_count(&self, db: &sqlx::PgPool) -> Result<i64, SimilariumError> {
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

    /// Get the guess contexts for a game
    ///
    /// If the order is by Rank, then the guess contexts are ordered by rank ascending and the
    /// username + profile_photo is of the first user that made the guess
    /// If the order is by GuessUpdated, then the guess contexts are ordered by updated descending
    /// and the username + profile_photo is of the last user that made the guess
    pub async fn get_guess_contexts(
        &self,
        order: GuessContextOrder,
        count: i64,
        db: &sqlx::PgPool,
    ) -> Result<Vec<GuessContext>, SimilariumError> {
        let q = format!(
            r#"
            SELECT
                word,
                guess_num,
                username,
                profile_photo,
                rank,
                similarity,
                (rank = 0) as is_secret
            FROM
                guess g
            LEFT JOIN
                "user" u
            ON
                {}
            WHERE
                game_id = $1
            ORDER BY
                {}
            LIMIT $2
            "#,
            match order {
                GuessContextOrder::Rank => "g.user_id = u.id",
                GuessContextOrder::GuessUpdated => "g.latest_guess_user_id = u.id",
            },
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

    /// Get the top guess rank
    ///
    /// This is useful to know if a new guess is going to be in the top X, by comparing what
    /// the top rank was with a new guess
    pub async fn get_top_guess_rank(
        &self,
        db: &sqlx::PgPool,
    ) -> Result<Option<i64>, SimilariumError> {
        sqlx::query!(
            r#"
            SELECT
                rank
            FROM
                guess
            WHERE
                game_id = $1
            ORDER BY
                rank ASC
            LIMIT 1
            "#,
            self.id
        )
        .fetch_optional(db)
        .await?
        .map_or(Ok(None), |g| Ok(Some(g.rank)))
    }

    pub async fn get_participant_user_ids(
        &self,
        db: &sqlx::PgPool,
    ) -> Result<Vec<String>, SimilariumError> {
        let user_ids: Vec<String> = sqlx::query!(
            r#"
            SELECT
                distinct(user_id)
            FROM
                guess
            WHERE
                game_id = $1
            "#,
            self.id
        )
        .fetch_all(db)
        .await?
        .iter()
        .map(|g| g.user_id.to_string())
        .collect();

        Ok(user_ids)
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

    pub async fn user_already_won(
        &self,
        user_id: &str,
        db: &sqlx::PgPool,
    ) -> Result<bool, SimilariumError> {
        Ok(GameWinnerAssociation::get(self.id, user_id, db)
            .await?
            .is_some())
    }

    pub async fn add_winner(
        &self,
        user_id: &str,
        guess_idx: i64,
        db: &sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        let game_winner = GameWinnerAssociation {
            game_id: self.id,
            user_id: user_id.to_string(),
            guess_idx,
            created: chrono::Utc::now().timestamp_millis(),
        };
        game_winner.insert(db).await?;

        Ok(())
    }

    pub async fn get_winners(
        &self,
        db: &sqlx::PgPool,
    ) -> Result<Vec<GameWinnerAssociation>, SimilariumError> {
        let winners = sqlx::query_as!(
            GameWinnerAssociation,
            r#"
            SELECT
                *
            FROM
                game_user_winner_association
            WHERE
                game_id = $1
            ORDER BY
                created ASC
            "#,
            self.id
        )
        .fetch_all(db)
        .await?;

        Ok(winners)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::channel::Channel;

    #[sqlx::test]
    async fn test_get_next_puzzle_number_with_no_games_is_1(
        pool: sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        let channel_id = "channel_id".to_string();

        let next_puzzle_number = Game::get_next_puzzle_number(channel_id, &pool).await;

        assert_eq!(next_puzzle_number, 1);

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_next_puzzle_number_increments_last_highest(
        pool: sqlx::PgPool,
    ) -> Result<(), SimilariumError> {
        // Insert a channel for the game
        let channel_id = "channel_id".to_string();
        let channel = Channel {
            id: channel_id.clone(),
            team_id: "team_id".to_string(),
            hour: 0,
            minute: 0,
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
            taunt_index: 0,
        };
        game.insert(&pool).await?;

        let next_puzzle_number = Game::get_next_puzzle_number(channel_id, &pool).await;

        assert_eq!(next_puzzle_number, 4);

        Ok(())
    }
}
