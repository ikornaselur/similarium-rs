use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SlackBot {
    pub id: Uuid,
    pub app_id: String,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub team_id: String,
    pub team_name: Option<String>,
    pub bot_token: Option<String>,
    pub bot_id: Option<String>,
    pub bot_user_id: Option<String>,
    pub bot_scopes: Option<String>,
    pub bot_refresh_token: Option<String>,
    pub bot_token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_enterprise_install: bool,
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

impl SlackBot {
    pub async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO 
                slack_bots(
                    app_id, 
                    enterprise_id, 
                    enterprise_name, 
                    team_id, 
                    team_name, 
                    bot_token, 
                    bot_id, 
                    bot_user_id, 
                    bot_scopes, 
                    bot_refresh_token, 
                    bot_token_expires_at, 
                    is_enterprise_install,
                    installed_at
                )
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, true, $12);
            "#,
            self.app_id,
            self.enterprise_id,
            self.enterprise_name,
            self.team_id,
            self.team_name,
            self.bot_token,
            self.bot_id,
            self.bot_user_id,
            self.bot_scopes,
            self.bot_refresh_token,
            self.bot_token_expires_at,
            self.installed_at,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_slack_bot_token(
        team_id: &str,
        api_app_id: &str,
        db: &sqlx::PgPool,
    ) -> Result<String, sqlx::Error> {
        // Get the bot token
        sqlx::query_scalar!(
            r#"
            SELECT
                bot_token
            FROM
                slack_bots
            WHERE
                team_id=$1 AND app_id=$2
            ORDER BY 
                installed_at DESC
            LIMIT 1;
            "#,
            team_id,
            api_app_id,
        )
        .fetch_one(db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
    }
}
