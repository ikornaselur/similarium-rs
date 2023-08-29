use crate::api::app::AppState;
use crate::game::submit_guess;
use crate::game::utils::get_game_blocks;
use crate::models::{Game, SlackBot};
use crate::payloads::{Event, EventPayload};
use crate::utils::get_or_create_user;
use crate::SimilariumError;
use actix_web::{post, web, HttpResponse, Scope};

#[post("")]
async fn post_events(
    event: web::Form<Event>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    // Parse the event.payload json string into EventPayload
    let payload = serde_json::from_str::<EventPayload>(&event.payload)?;

    match payload {
        EventPayload {
            actions,
            user,
            api_app_id,
            channel,
            message,
            ..
        } if actions.len() == 1 => {
            let action = actions.first().unwrap();
            if action.action_id != "submit-guess" {
                return Err(SimilariumError::validation_error(
                    format!("Invalid action_id: {}", action.action_id).as_str(),
                ));
            }
            let local_user = get_or_create_user(
                &user.id.clone(),
                &user.team_id.clone(),
                &api_app_id,
                &app_state.db,
                &app_state.slack_client,
            )
            .await?;

            let game = Game::get(channel.id.as_str(), message.ts.as_str(), &app_state.db)
                .await?
                .ok_or_else(|| SimilariumError::validation_error("Game not found"))?;

            submit_guess(&local_user, &game, &action.value, &app_state).await?;

            let blocks = get_game_blocks(game, &app_state.db).await?;
            let token =
                SlackBot::get_slack_bot_token(&user.team_id, &api_app_id, &app_state.db).await?;

            let _ = &app_state
                .slack_client
                .chat_update(
                    "Update to today's game",
                    &channel.id,
                    &message.ts,
                    &token,
                    Some(blocks),
                )
                .await?;
        }
        _ => {
            todo!("Not handled");
        }
    }

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/events").service(post_events)
}
