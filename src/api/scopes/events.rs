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
                return validation_error!("Invalid action_id: {}", action.action_id);
            }
            let local_user = get_or_create_user(
                &user.id,
                &user.team_id,
                &api_app_id,
                &app_state.db,
                &app_state.slack_client,
            )
            .await?;

            let game = Game::get(channel.id.as_str(), message.ts.as_str(), &app_state.db)
                .await?
                .map_or_else(|| validation_error!("Game not found"), Ok)?;

            let guess = submit_guess(&local_user, &game, &action.value, &app_state).await?;

            // TODO: prevent guesses after getting the secret
            if guess.is_secret() {
                let token =
                    SlackBot::get_slack_bot_token(&user.team_id, &api_app_id, &app_state.db)
                        .await?;
                // Let the user know they guessed the secret
                app_state
                    .slack_client
                    .post_ephemeral(
                        &format!(
                            ":tada: You found the secret! It was *{}* :tada:",
                            guess.word
                        ),
                        &channel.id,
                        &user.id,
                        &token,
                        None,
                    )
                    .await?;

                // Post on the channel to celebrate!
                let celebrate_emoji = ":tada:";
                app_state
                    .slack_client
                    .post_message(
                        &format!(
                            "{} <@{}> has just found the secret of the day! {}",
                            celebrate_emoji, user.id, celebrate_emoji
                        ),
                        &channel.id,
                        &token,
                        None,
                    )
                    .await?;
            }

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
