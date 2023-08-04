use chrono::NaiveDateTime;
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
    pub bot_token_expires_at: Option<NaiveDateTime>,
    pub is_enterprise_install: bool,
    pub installed_at: NaiveDateTime,
}

pub async fn insert_slack_bot(slack_bot: SlackBot, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO 
            slack_bots(app_id, enterprise_id, enterprise_name, team_id, team_name, bot_token, bot_id, bot_user_id, bot_scopes, bot_refresh_token, bot_token_expires_at, is_enterprise_install, installed_at)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);
        "#,
        slack_bot.app_id,
        slack_bot.enterprise_id,
        slack_bot.enterprise_name,
        slack_bot.team_id,
        slack_bot.team_name,
        slack_bot.bot_token,
        slack_bot.bot_id,
        slack_bot.bot_user_id,
        slack_bot.bot_scopes,
        slack_bot.bot_refresh_token,
        slack_bot.bot_token_expires_at,
        slack_bot.is_enterprise_install,
        slack_bot.installed_at,
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_slack_bot_token(
    team_id: &str,
    api_app_id: &str,
    db: &sqlx::PgPool,
) -> Result<Option<String>, sqlx::Error> {
    // Get the bot token
    let token = sqlx::query_scalar!(
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
    .await?;

    Ok(token)
}
