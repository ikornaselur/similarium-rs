use crate::game::utils::get_header_text;
use chrono::{NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::{
    game::utils::{get_game_blocks, get_secret},
    models::{Channel, Game, Word2Vec},
    payloads::CommandPayload,
    slack_client::{responses::UserInfoResponse, SlackClient, SlackMessage, SlackUserDetails},
    utils::{get_utc_naive_time, when_human},
    SimilariumError, SimilariumErrorType,
};

pub async fn schedule_game_on_channel(
    db: &sqlx::PgPool,
    slack_client: &(impl SlackUserDetails + SlackMessage),
    payload: &CommandPayload,
    token: &str,
    time: NaiveTime,
) -> Result<(), SimilariumError> {
    // Get channel
    let channel = Channel::get(&payload.channel_id, db).await?;
    log::debug!("Found channel: {:?}", channel);

    // Check if channel exists and is active, error if so
    if channel.as_ref().is_some_and(|channel| channel.active) {
        log::debug!("Channel is active, erroring");
        return validation_error!(
            ":no_entry_sign: Game is already registered for the channel. \
             Please use the \"stop\" command before running \"start\" again."
        );
    }

    // Get user info for timezone
    let user = match slack_client.get_user_details(&payload.user_id, token).await {
        Ok(UserInfoResponse {
            user: Some(user), ..
        }) => user,
        _ => {
            return slack_api_error!("Error fetching user details");
        }
    };

    // Convert the given time to UTC, using the timezone offset
    let utc_time = get_utc_naive_time(time, user.tz_offset);
    let when = when_human(time);
    let tz_offset = match user.tz_offset {
        0 => "UTC".to_string(),
        offset if offset < 0 => format!("UTC-{}", offset / 3600),
        offset => format!("UTC+{}", offset / 3600),
    };

    log::info!("Starting game on channel {}: {}", payload.channel_id, when);

    // Post that the game is starting, cathing an error if slack fails, so that we can tell the
    // user that Similarium doesn't have the required permissions
    match slack_client
        .post_message(
            &format!(
                "<@{}> has started a daily game of Similarium {} {}",
                user.id, when, tz_offset
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
            channel.hour = utc_time.hour() as i32;
            channel.minute = utc_time.minute() as i32;
            channel.update(db).await?;
        }
        None => {
            log::debug!("Channel not found, creating...");
            let channel = Channel {
                id: payload.channel_id.clone(),
                team_id: payload.team_id.clone(),
                hour: utc_time.hour() as i32,
                minute: utc_time.minute() as i32,
                active: true,
            };
            channel.insert(db).await?;
        }
    };

    Ok(())
}

pub async fn stop_games_on_channel(
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
    payload: &CommandPayload,
    token: &str,
) -> Result<(), SimilariumError> {
    match Channel::get(&payload.channel_id, db).await {
        Ok(None) => {
            log::debug!("No game was registered");
            return validation_error!(
                ":no_entry_sign: No game is registered for the channel, did you mean to run \"start\"?"
            );
        }
        Ok(Some(mut channel)) => {
            log::debug!("Setting {} as inactive", channel.id);
            channel.active = false;
            channel.update(db).await?;

            let message = format!(
                "<@{}> has stopped the daily game of Similarium",
                payload.user_id
            );
            slack_client
                .post_message(&message, &payload.channel_id, token, None)
                .await?;
        }
        Err(e) => {
            log::error!("Error fetching channel: {}", e);
            return db_error!("Unable to fetch channel");
        }
    }

    Ok(())
}

pub async fn manual_start(
    payload: &CommandPayload,
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
    channel_id: &str,
    token: &str,
) -> Result<(), SimilariumError> {
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
                minute: 0,
                active: true,
            };
            channel.insert(db).await?;
            channel
        }
    };

    log::debug!("Setting up target word");
    let datetime = Utc::now();
    let puzzle_number = Game::get_next_puzzle_number(channel.id.clone(), db).await;

    let secret = get_secret(&channel.id, puzzle_number);
    let target_word = Word2Vec {
        word: secret.clone(),
    };
    target_word.create_materialised_view(db).await?;
    log::debug!("Target word: {}", target_word.word);

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
    let blocks = get_game_blocks(&game, db).await?;

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

    Ok(())
}

pub async fn start_game_on_channel(
    db: &sqlx::PgPool,
    slack_client: &impl SlackMessage,
    channel_id: &str,
    token: &str,
) -> Result<(), SimilariumError> {
    let channel = match Channel::get(channel_id, db).await? {
        Some(channel) => channel,
        None => {
            log::error!(
                "Unable to start game on channel ({}): channel not found",
                channel_id
            );
            return validation_error!("Channel not found");
        }
    };

    let datetime = Utc::now();
    let puzzle_number = Game::get_next_puzzle_number(channel.id.clone(), db).await;

    let secret = get_secret(&channel.id, puzzle_number);
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
    let blocks = get_game_blocks(&game, db).await?;
    let text = get_header_text(game.date, game.puzzle_number);

    log::debug!("Submitting the message");
    let res = slack_client
        .post_message(&text, channel_id, token, Some(blocks))
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

    Ok(())
}

pub async fn get_active_games_on_channel(
    db: &sqlx::PgPool,
    channel_id: &str,
) -> Result<Vec<Game>, SimilariumError> {
    let channel = match Channel::get(channel_id, db).await? {
        Some(channel) => channel,
        None => {
            return validation_error!("Channel not found");
        }
    };

    channel.get_active_games(db).await
}

pub async fn end_game(
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
    game: &mut Game,
    token: &str,
) -> Result<(), SimilariumError> {
    log::debug!("Ending game: {}", game.id);

    // Set game to be inactive
    game.set_active(false, db).await?;
    if let Some(thread_ts) = &game.thread_ts {
        // Update the game to say it's over
        let blocks = get_game_blocks(game, db).await?;

        slack_client
            .chat_update(
                "Update to today's game",
                &game.channel_id,
                thread_ts,
                token,
                Some(blocks),
            )
            .await?;
    }

    Ok(())
}
