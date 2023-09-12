use actix_web::{post, web, HttpResponse, Scope};
use chrono::Utc;
use uuid::Uuid;

use crate::api::app::AppState;
use crate::api::utils::{parse_command, Command};
use crate::game::utils::{get_header_body, get_header_text, get_secret};
use crate::models::{Channel, Game, SlackBot, Word2Vec};
use crate::payloads::CommandPayload;
use crate::slack_client::{Block, SlackClient};
use crate::{SimilariumError, SimilariumErrorType};

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    let payload = form.into_inner();
    let token =
        SlackBot::get_slack_bot_token(&payload.team_id, &payload.api_app_id, &app_state.db).await?;
    let command = parse_command(&payload.text)?;

    match command {
        Command::Help => {
            app_state
                .slack_client
                .post_message("Help text", &payload.channel_id, &token, None)
                .await?;
        }
        Command::Start(time) => {
            app_state
                .slack_client
                .post_message(
                    &format!("Starting the game at {}", time.format("%H:%M")),
                    &payload.channel_id,
                    &token,
                    None,
                )
                .await?;
        }
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
        Command::Invalid(message) => {
            app_state
                .slack_client
                .post_message(&message, &payload.channel_id, &token, None)
                .await?;
        }
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
                channel.set_active(true, db).await?;
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
        Block::section(&header_body),
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
