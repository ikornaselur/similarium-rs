use crate::api::app::AppState;
use crate::models::slack_bot::get_slack_bot_token;
use crate::payloads::CommandPayload;
use crate::SimilariumError;
use actix_web::{post, web, HttpResponse, Scope};

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    log::debug!("POST /slack/similarium");

    let payload = form.into_inner();

    let token = get_slack_bot_token(&payload.team_id, &payload.api_app_id, &app_state.db).await?;

    match payload.text.to_lowercase().trim() {
        "help" => {
            app_state
                .slack_client
                .post_message("Help text", &payload.channel_id, &token)
                .await?;
        }
        _ => {
            app_state
                .slack_client
                .post_message(
                    &format!("Hey <@{}>! You said: {}", payload.user_id, payload.text),
                    &payload.channel_id,
                    &token,
                )
                .await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
