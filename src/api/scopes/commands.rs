use crate::api::app::AppState;
use crate::api::payloads::CommandPayload;
use crate::slack_client::SlackClient;
use crate::SimilariumError;
use actix_web::{post, web, HttpResponse, Scope};

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    log::debug!("POST /slack/similarium");

    let payload = form.into_inner();

    let client = SlackClient::new(
        payload.team_id.clone(),
        payload.api_app_id.clone(),
        payload.channel_id.clone(),
        &app_state.db,
    )
    .await?;

    match payload.text.to_lowercase().trim() {
        "help" => {
            client.post_message("Help text".to_string()).await?;
        }
        _ => {
            client
                .post_message(format!(
                    "Hey <@{}>! You said: {}",
                    payload.user_id, payload.text
                ))
                .await?
        }
    }

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
