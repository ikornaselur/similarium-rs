use crate::{api::app::AppState, models::SlackBot, slack_client::SlackOAuth, SimilariumError};
use actix_web::{get, web, HttpResponse, Scope};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OAuthRedirect {
    code: String,
}

#[get("/oauth_redirect")]
async fn get_oauth_redirect(
    info: web::Query<OAuthRedirect>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    let code = &info.code;

    let payload = app_state
        .slack_client
        .post_oauth_code(
            code,
            &app_state.config.slack_client_id,
            &app_state.config.slack_client_secret,
        )
        .await?;

    let slack_bot = SlackBot {
        id: uuid::Uuid::new_v4(),
        app_id: payload.app_id,
        team_id: payload.team.id.clone(),
        team_name: payload.team.name.clone(),
        bot_token: payload.access_token,
        bot_user_id: payload.bot_user_id,
        bot_scopes: payload.scope,
        is_enterprise_install: payload.is_enterprise_install,
        installed_at: chrono::Utc::now(),
        bot_id: None,
        bot_refresh_token: None,
        bot_token_expires_at: None,
        enterprise_id: None,
        enterprise_name: None,
    };
    slack_bot.insert(&app_state.db).await?;

    // Redirect the user
    Ok(HttpResponse::Ok().body("Auth, sweet auth!"))
}

pub fn scope() -> Scope {
    web::scope("/auth").service(get_oauth_redirect)
}
