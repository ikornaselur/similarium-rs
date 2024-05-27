use crate::{
    ai::get_celebration,
    api::app::AppState,
    game::{submit_guess, utils::get_game_blocks},
    models::{Game, Guess, SlackBot},
    payloads::{Channel, Event, EventPayload, User},
    slack_client::SlackMessage,
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
            let guess_value = action.value.trim();
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

            // Get the top rank, so that we can know if we had a milestone with this guess. A
            // milestone is first guess in the top 1000, top 100 and top 10.
            // If no guess has been made, we can just set the top rank to 1001, as that's higher
            // than the biggest 'bucket' we celebrate for
            let top_rank = game
                .get_top_guess_rank(&app_state.db)
                .await?
                .unwrap_or(1001);

            // Match on SimilariumError with error_type SimilariumErrorType::NotFound to let the
            // user know the word isn't in the dictionary
            let guess = match submit_guess(&local_user, &game, guess_value, &app_state.db).await {
                Ok(guess) => guess,
                Err(SimilariumError {
                    error_type: crate::error::SimilariumErrorType::NotFound,
                    ..
                }) => {
                    app_state
                        .slack_client
                        .post_ephemeral(
                            &format!(
                                ":warning: *\"{}\" is not a valid word!* :warning:",
                                &guess_value,
                            ),
                            &channel.id,
                            &user.id,
                            &token,
                            None,
                        )
                        .await?;
                    return Ok(HttpResponse::Ok().into());
                }
                Err(e) => return Err(e),
            };

            let is_secret = guess.is_secret();
            if is_secret {
                let guess_num = match guess {
                    Guess {
                        ref user_id,
                        guess_num: Some(guess_num),
                        ..
                    } if user_id == &user.id => guess_num,
                    _ => game.get_guess_count(&app_state.db).await.unwrap_or(0),
                };
                game.add_winner(&user.id, guess_num, &app_state.db).await?;

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

            if !is_secret && top_rank > 10 && guess.rank <= 1000 && guess.rank < top_rank {
                celebrate(top_rank, guess, game, user, app_state, channel, token).await?;
            }
        }
        _ => {
            todo!("Not handled");
        }
    }

    Ok(HttpResponse::Ok().into())
}

async fn celebrate(
    top_rank: i64,
    guess: Guess,
    game: Game,
    user: User,
    app_state: web::Data<AppState>,
    channel: Channel,
    token: String,
) -> Result<(), SimilariumError> {
    let mut should_celebrate = false;
    let mut bucket = i64::MAX;

    if guess.rank <= 10 && top_rank > 10 {
        should_celebrate = true;
        bucket = 10;
    } else if guess.rank <= 100 && top_rank > 100 {
        should_celebrate = true;
        bucket = 100;
    } else if guess.rank <= 1000 && top_rank > 1000 {
        should_celebrate = true;
        bucket = 1000;
    }

    if !should_celebrate {
        return Ok(());
    }

    let celebration = get_celebration(
        game.get_guess_count(&app_state.db).await.unwrap_or(0),
        &user.id,
        &guess.word,
        guess.rank,
        bucket,
    )
    .await?;
    app_state
        .slack_client
        .post_message(&celebration.message, &channel.id, &token, None)
        .await?;

    Ok(())
}

pub fn scope() -> Scope {
    web::scope("/events").service(post_events)
}
