use crate::models::Game;
use crate::SimilariumError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    pub id: String,
    pub team_id: String,
    pub hour: i32,
    pub minute: i32,
    pub active: bool,
}

impl Channel {
    pub async fn get(
        channel_id: &str,
        db: &sqlx::PgPool,
    ) -> Result<Option<Channel>, SimilariumError> {
        let channel = sqlx::query_as!(
            Channel,
            r#"
            SELECT 
                * 
            FROM
                channel 
            WHERE
                id = $1
            "#,
            channel_id
        )
        .fetch_optional(db)
        .await?;

        Ok(channel)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), SimilariumError> {
        sqlx::query!(
            r#"
            INSERT INTO
                channel(id, team_id, hour, minute, active)
            VALUES ($1, $2, $3, $4, $5);
            "#,
            self.id,
            self.team_id,
            self.hour,
            self.minute,
            self.active,
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /// Update the channel in the database
    ///
    /// Updates:
    ///     * active
    ///     * hour
    ///     * minute
    ///
    /// Does not update:
    ///     * id
    ///     * team_id
    pub async fn update(&mut self, db: &sqlx::PgPool) -> Result<(), SimilariumError> {
        sqlx::query!(
            r#"
            UPDATE 
                channel
            SET 
                active = $1,
                hour = $2,
                minute = $3
            WHERE
                id = $4
            "#,
            self.active,
            self.hour,
            self.minute,
            self.id,
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_active_games(&self, db: &sqlx::PgPool) -> Result<Vec<Game>, SimilariumError> {
        log::debug!("Fetching active games for channel: {}", self.id);

        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT 
                *
            FROM
                game
            WHERE
                channel_id = $1 AND
                active = true
            "#,
            self.id
        )
        .fetch_all(db)
        .await?;

        Ok(games)
    }

    pub async fn get_channels_for_hour_minute(
        hour: i32,
        minute: i32,
        db: &sqlx::PgPool,
    ) -> Result<Vec<Channel>, SimilariumError> {
        let channels = sqlx::query_as!(
            Channel,
            r#"
            SELECT
                *
            FROM
                channel
            WHERE
                hour = $1 AND
                minute = $2 AND
                active = true
            "#,
            hour,
            minute
        )
        .fetch_all(db)
        .await?;

        Ok(channels)
    }
}
