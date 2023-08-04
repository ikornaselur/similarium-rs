use crate::api::app::AppState;
use crate::api::responses::SlackOAuthResponse;
use crate::models::slack_bot::insert_slack_bot;
use crate::models::SlackBot;
use actix_web::{error, get, web, Error, HttpResponse, Scope};
use serde::Deserialize;

const OAUTH_API_URL: &str = "https://slack.com/api/oauth.v2.access";

#[derive(Deserialize, Debug)]
struct OAuthRedirect {
    code: String,
}

#[get("/oauth_redirect")]
async fn get_oauth_redirect(
    info: web::Query<OAuthRedirect>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    log::debug!("GET /auth/oauth_redirect");

    let code = &info.code;

    // Post the code, along with client id and secret, to Slack's OAuth API
    let client = awc::Client::default();
    let mut res = client
        .post(OAUTH_API_URL)
        .send_form(&[
            ("code", code),
            ("client_id", &app_state.config.slack_client_id),
            ("client_secret", &app_state.config.slack_client_secret),
        ])
        .await
        .map_err(|e| {
            log::error!("Error posting to Slack OAuth API: {}", e);
            error::ErrorInternalServerError("OAuth failed")
        })?;

    let payload: SlackOAuthResponse = res.json().await.map_err(|e| {
        log::error!("Error parsing Slack OAuth response: {}", e);
        error::ErrorInternalServerError("OAuth failed")
    })?;

    let slack_bot = SlackBot {
        id: uuid::Uuid::new_v4(),
        app_id: payload.app_id,
        team_id: payload.team.id.clone(),
        team_name: payload.team.name.clone(),
        bot_token: payload.access_token,
        bot_user_id: payload.bot_user_id,
        bot_scopes: payload.scope,
        is_enterprise_install: payload.is_enterprise_install,
        installed_at: chrono::Utc::now().naive_utc(),
        bot_id: None,
        bot_refresh_token: None,
        bot_token_expires_at: None,
        enterprise_id: None,
        enterprise_name: None,
    };

    insert_slack_bot(slack_bot, &app_state.db)
        .await
        .map_err(|e| {
            log::error!("Error inserting Slack bot into database: {}", e);
            error::ErrorInternalServerError("OAuth failed")
        })?;

    // Redirect the user
    Ok(HttpResponse::Ok().body("Auth, sweet auth!"))
}

pub fn scope() -> Scope {
    web::scope("/auth").service(get_oauth_redirect)
}
