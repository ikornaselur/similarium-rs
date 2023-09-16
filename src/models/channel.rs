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
    pub async fn get(channel_id: &str, db: &sqlx::PgPool) -> Result<Option<Channel>, sqlx::Error> {
        let channel = sqlx::query_as!(
            Channel,
            r#"
            SELECT * FROM
                channel 
            WHERE
                id = $1;
            "#,
            channel_id
        )
        .fetch_optional(db)
        .await?;

        Ok(channel)
    }

    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO
                channel(id, team_id, hour, active)
            VALUES ($1, $2, $3, $4);
            "#,
            self.id,
            self.team_id,
            self.hour,
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
    ///
    /// Does not update:
    ///     * id
    ///     * team_id
    pub async fn update(&mut self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE 
                channel
            SET 
                hour = $1,
                active = $2
            WHERE
                id = $3;
            "#,
            self.hour,
            self.active,
            self.id,
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
