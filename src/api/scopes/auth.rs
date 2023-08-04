use crate::api::app::{AppState, Config};
use crate::api::responses::Team;
use actix_web::{get, web, Error, HttpResponse, Scope};
use serde::Deserialize;

const OAUTH_API_URL: &str = "https://slack.com/api/oauth.v2.access";

#[derive(Deserialize, Debug)]
struct OAuthRedirect {
    code: String,
}

#[derive(Debug, Deserialize)]
struct SlackOAuthPayload {
    ok: bool,
    error: Option<String>,
    access_token: Option<String>,
    scope: Option<String>,
    bot_user_id: Option<String>,
    app_id: Option<String>,
    team: Option<Team>,
    is_enterprise_install: Option<bool>,
}

#[get("/oauth_redirect")]
async fn get_oauth_redirect(
    info: web::Query<OAuthRedirect>,
    app_state: web::Data<AppState>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    log::debug!("GET /auth/oauth_redirect");

    let code = &info.code;

    // Post the code, along with client id and secret, to Slack's OAuth API
    let client = awc::Client::default();
    let mut res = client
        .post(OAUTH_API_URL)
        .send_form(&[
            ("code", code),
            ("client_id", &config.slack_client_id),
            ("client_secret", &config.slack_client_secret),
        ])
        .await
        .unwrap();

    let payload: SlackOAuthPayload = res.json().await.unwrap();

    if !payload.ok {
        log::error!(
            "OAuth failed: {}",
            payload.error.unwrap_or("Unknwon error".to_string())
        );
        return Ok(HttpResponse::Ok().body("OAuth failed"));
    }

    sqlx::query!(
        r#"
        INSERT INTO 
            slack_bots(client_id, app_id, team_id, team_name, bot_token, bot_user_id, bot_scopes, is_enterprise_install, installed_at)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9);
        "#,
        config.slack_client_id,
        payload.app_id.unwrap(),
        payload.team.as_ref().unwrap().id,
        payload.team.as_ref().unwrap().name.as_ref().unwrap(),
        payload.access_token.unwrap(),
        payload.bot_user_id.unwrap(),
        payload.scope.unwrap(),
        payload.is_enterprise_install.unwrap(),
        sqlx::types::chrono::Utc::now().naive_utc(),
    ).execute(&app_state.db).await.unwrap();

    // Redirect the user
    Ok(HttpResponse::Ok().body("Auth, sweet auth!"))
}

pub fn auth() -> Scope {
    web::scope("auth").service(get_oauth_redirect)
}
