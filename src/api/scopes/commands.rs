use crate::api::app::AppState;
use crate::api::payloads::CommandPayload;
use crate::models::slack_bot::get_slack_bot_token;
use actix_web::{error, post, web, Error, HttpResponse, Scope};

const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";

#[post("/{command}")]
async fn post_commands(
    path: web::Path<String>,
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let command = path.into_inner();
    let payload = form.into_inner();

    log::debug!("POST /slack/commands/{}", command);

    // Get the bot token
    let token = get_slack_bot_token(&payload.team_id, &payload.api_app_id, &app_state.db)
        .await
        .map_err(|e| {
            log::error!("Error getting bot token: {}", e);
            error::ErrorNotFound("Installation not found for team")
        })?;

    // Post a message, with the text, to the channel using the post message endpoint
    let client = awc::Client::default();
    client
        .post(POST_MESSAGE_URL)
        .send_form(&[
            ("token", token),
            ("channel", Some(payload.channel_id)),
            (
                "text",
                Some(format!(
                    "Hey <@{}>! You said: {}",
                    payload.user_id, payload.text
                )),
            ),
        ])
        .await
        .map_err(|e| {
            log::error!("Error posting to Slack API: {}", e);
            error::ErrorInternalServerError("Error posting message")
        })?;

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_commands)
}
