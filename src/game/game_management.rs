use chrono::{NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::game::utils::{get_header_body, get_header_text, get_secret};
use crate::models::{Channel, Game, Word2Vec};
use crate::payloads::CommandPayload;
use crate::slack_client::responses::UserInfoResponse;
use crate::slack_client::{Block, SlackClient};
use crate::utils::{get_hour, when_human};
use crate::{SimilariumError, SimilariumErrorType};

pub async fn start_game(
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
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
    let hour = get_hour(time, user.tz_offset);
    let when_human = when_human(time.hour());
    let tz_offset = if user.tz_offset < 0 {
        format!("UTC-{}", user.tz_offset / 3600)
    } else {
        format!("UTC+{}", user.tz_offset / 3600)
    };

    // Post that the game is starting, cathing an error if slack fails, so that we can tell the
    // user that Similarium doesn't have the required permissions
    match slack_client
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
            channel.update(db).await?;
        }
        None => {
            log::debug!("Channel not found, creating...");
            let channel = Channel {
                id: payload.channel_id.clone(),
                team_id: payload.team_id.clone(),
                hour: hour.into(),
                active: true,
            };
            channel.insert(db).await?;
        }
    };

    Ok(())
}

pub async fn stop_game(
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
