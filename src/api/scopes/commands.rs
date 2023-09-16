use actix_web::{post, web, HttpResponse, Scope};
use chrono::{NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::api::app::AppState;
use crate::api::utils::{parse_command, Command};
use crate::game::utils::{get_header_body, get_header_text, get_help_blocks, get_secret};
use crate::models::{Channel, Game, SlackBot, Word2Vec};
use crate::payloads::CommandPayload;
use crate::slack_client::responses::UserInfoResponse;
use crate::slack_client::{Block, SlackClient};
use crate::utils::{get_hour, when_human};
use crate::{SimilariumError, SimilariumErrorType};

async fn start_game(
    app_state: &web::Data<AppState>,
    payload: &CommandPayload,
    token: &str,
    time: NaiveTime,
) -> Result<(), SimilariumError> {
    // Get channel
    let channel = Channel::get(&payload.channel_id, &app_state.db).await?;

    // Check if channel exists and is active, error if so
    if channel.as_ref().is_some_and(|channel| channel.active) {
        return validation_error!(
            ":no_entry_sign: Game is already registered for the channel. \
             Please use the \"stop\" command before running \"start\" again."
        );
    }

    // Get user info for timezone
    let user = match app_state
        .slack_client
        .get_user_details(&payload.user_id, token)
        .await
    {
        Ok(UserInfoResponse {
            user: Some(user), ..
        }) => user,
        _ => {
            return slack_api_error!("Error fetching user details");
        }
    };

    // Convert the given time to UTC, using the timezone offset
    let hour = get_hour(time, user.tz_offset);
    let when_human = when_human(time.hour());
    let tz_offset = if user.tz_offset < 0 {
        format!("UTC-{}", user.tz_offset / 3600)
    } else {
        format!("UTC+{}", user.tz_offset / 3600)
    };

    // Post that the game is starting, cathing an error if slack fails, so that we can tell the
    // user that Similarium doesn't have the required permissions
    match app_state
        .slack_client
        .post_message(
            &format!(
                "<@{}> has started a daily game of Similarium {} {}",
                user.id, when_human, tz_offset
            ),
            &payload.channel_id,
            token,
            None,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            // TODO: Explore how this works .. as we're responding back ephemerally with the
            // channel_id, which will fail because the bot isn't in the channel? But this works in
            // the Python version
            log::error!("Error posting to Slack API: {}", e);
            return slack_api_error!(
                ":no_entry_sign: Unable to post to channel. You need to \
                 invite @Similarium to this channel: `/invite @Similarium`"
            );
        }
    };

    // Create the channel if it doesn't exist, otherwise update active + time
    match channel {
        Some(mut channel) => {
            channel.active = true;
            channel.hour = hour.into();
            channel.update(&app_state.db).await?;
        }
        None => {
            log::debug!("Channel not found, creating...");
            let channel = Channel {
                id: payload.channel_id.clone(),
                team_id: payload.team_id.clone(),
                hour: hour.into(),
                active: true,
            };
            channel.insert(&app_state.db).await?;
        }
    };

    Ok(())
}

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    let payload = form.into_inner();
    let token =
        SlackBot::get_slack_bot_token(&payload.team_id, &payload.api_app_id, &app_state.db).await?;
    let command = match parse_command(&payload.text) {
        Ok(command) => command,
        Err(e) => {
            app_state
                .slack_client
                .post_ephemeral(
                    &e.message.unwrap(),
                    &payload.channel_id,
                    &payload.user_id,
                    &token,
                    None,
                )
                .await?;
            return Ok(HttpResponse::Ok().into());
        }
    };

    match command {
        Command::Help => {
            let help_blocks = get_help_blocks();
            app_state
                .slack_client
                .post_ephemeral(
                    "Hello!",
                    &payload.channel_id,
                    &payload.user_id,
                    &token,
                    Some(help_blocks),
                )
                .await?;
        }
        Command::Start(time) => match start_game(&app_state, &payload, &token, time).await {
            Ok(_) => {}
            Err(e) => {
                app_state
                    .slack_client
                    .post_ephemeral(
                        &e.message.unwrap(),
                        &payload.channel_id,
                        &payload.user_id,
                        &token,
                        None,
                    )
                    .await?;
            }
        },
        Command::ManualStart => {
            manual_start(
                &payload,
                &app_state.db,
                &app_state.slack_client,
                &payload.channel_id,
                &token,
            )
            .await?
        }
        Command::ManualEnd => todo!(),
        Command::Debug => todo!(),
        Command::Stop => todo!(),
    }

    Ok(HttpResponse::Ok().into())
}

async fn manual_start(
    payload: &CommandPayload,
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
    channel_id: &str,
    token: &str,
) -> Result<(), SimilariumError> {
    log::info!("Sending test blocks");

    // Get, or create, the channel
    let channel = match Channel::get(&payload.channel_id, db).await? {
        Some(mut channel) => {
            log::debug!("Found channel: {:?}", channel);
            if !channel.active {
                channel.active = true;
                channel.update(db).await?;
            }
            channel
        }
        None => {
            log::debug!("Channel not found, creating...");
            let channel = Channel {
                id: payload.channel_id.clone(),
                team_id: payload.team_id.clone(),
                hour: 0,
                active: true,
            };
            channel.insert(db).await?;
            channel
        }
    };

    log::debug!("Setting up target word");

    let datetime = Utc::now();
    let puzzle_number = Game::get_next_puzzle_number(channel.id.clone(), db).await;
    let header_text = get_header_text(datetime, puzzle_number);

    let secret = get_secret(channel.id.clone(), puzzle_number);
    let target_word = Word2Vec {
        word: secret.clone(),
    };
    target_word.create_materialised_view(db).await?;

    log::debug!("Setting up the game");
    let mut game = Game {
        id: Uuid::new_v4(),
        channel_id: channel.id.clone(),
        thread_ts: None,
        puzzle_number,
        date: datetime,
        active: true,
        hint: None,
        secret: target_word.word.clone(),
    };
    game.insert(db).await?;

    log::debug!("Setting up the message");
    let header_body = get_header_body(game.get_guess_count(db).await?);

    let blocks: Vec<Block> = vec![
        Block::header(&header_text),
        Block::section(&header_body, None),
        Block::divider(),
        Block::guess_input(),
    ];

    log::debug!("Submitting the message");
    let res = slack_client
        .post_message("Manual start", channel_id, token, Some(blocks))
        .await?;

    // Get the thread_ts and update the game
    let thread_ts = res
        .get("ts")
        .ok_or(SimilariumError {
            message: Some("Could not get thread_ts from Slack response".to_string()),
            error_type: SimilariumErrorType::MissingThreadTs,
        })?
        .as_str()
        .ok_or(SimilariumError {
            message: Some("Could not get thread_ts from Slack response".to_string()),
            error_type: SimilariumErrorType::MissingThreadTs,
        })?;

    game.set_thread_ts(thread_ts, db).await?;

    log::info!("Successfully sent test blocks");
    Ok(())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
