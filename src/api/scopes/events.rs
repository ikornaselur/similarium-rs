use crate::{
    api::app::AppState,
    game::{submit_guess, utils::get_game_blocks},
    models::{Game, SlackBot},
    payloads::{Event, EventPayload},
    utils::get_or_create_user,
    SimilariumError,
};
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
                &app_state.db,
                &app_state.slack_client,
            )
            .await?;
            let token = SlackBot::get_slack_bot_token(&user.team_id, &app_state.db).await?;

            let game = Game::get(channel.id.as_str(), message.ts.as_str(), &app_state.db)
                .await?
                .map_or_else(|| validation_error!("Game not found"), Ok)?;

            if game.user_already_won(&user.id, &app_state.db).await? {
                app_state
                    .slack_client
                    .post_ephemeral(
                        ":warning: You already got the winning word, you can't make any further guesses :warning:",
                        &channel.id,
                        &user.id,
                        &token,
                        None,
                    )
                    .await?;
                return Ok(HttpResponse::Ok().into());
            }

            let guess = submit_guess(&local_user, &game, &action.value, &app_state).await?;

            if guess.is_secret() {
                game.add_winner(&user.id, guess.guess_num.unwrap(), &app_state.db)
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

            let blocks = get_game_blocks(&game, &app_state.db).await?;

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
