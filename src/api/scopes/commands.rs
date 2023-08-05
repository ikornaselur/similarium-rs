use crate::api::app::AppState;
use crate::api::utils::{parse_command, Command};
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
    let command = parse_command(&payload.text)?;

    match command {
        Command::Help => {
            app_state
                .slack_client
                .post_message("Help text", &payload.channel_id, &token)
                .await?;
        }
        Command::Start(time) => {
            app_state
                .slack_client
                .post_message(
                    &format!("Starting the game at {}", time.format("%H:%M")),
                    &payload.channel_id,
                    &token,
                )
                .await?;
        }
        Command::ManualStart | Command::ManualEnd => todo!(),
        Command::Stop => todo!(),
        Command::Invalid(message) => {
            app_state
                .slack_client
                .post_message(&message, &payload.channel_id, &token)
                .await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
