use crate::api::app::AppState;
use crate::api::utils::{parse_command, Command};
use crate::game::utils::{get_header_body, get_header_text};
use crate::models::{SlackBot, Word2Vec};
use crate::payloads::CommandPayload;
use crate::slack_client::{Block, SlackClient};
use crate::SimilariumError;
use actix_web::{post, web, HttpResponse, Scope};
use time::macros::{datetime, format_description};

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    log::debug!("POST /slack/similarium");

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
            let format = format_description!("[hour]:[minute]");
            app_state
                .slack_client
                .post_message(
                    &format!("Starting the game at {}", time.format(&format)?),
                    &payload.channel_id,
                    &token,
                    None,
                )
                .await?;
        }
        Command::ManualStart => {
            test_blocks(
                &app_state.db,
                &app_state.slack_client,
                &payload.channel_id,
                &token,
            )
            .await?
        }
        Command::ManualEnd => todo!(),
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

async fn test_blocks(
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
    channel_id: &str,
    token: &str,
) -> Result<(), SimilariumError> {
    log::info!("Sending test blocks");

    let datetime = datetime!(2023-08-08 00:00:00 UTC);

    let target_word = Word2Vec {
        word: "car".to_string(),
    };
    target_word.create_materialised_view(db).await?;
    let (top, top10, top1000) = target_word.get_top_hints(db).await?;

    let header_text = get_header_text(datetime).unwrap();
    let header_body = get_header_body(top, top10, top1000);

    let blocks: Vec<Block> = vec![
        Block::header(&header_text),
        Block::section(&header_body),
        Block::divider(),
        Block::guess_input(),
    ];

    blocks.iter().for_each(|block| log::debug!("{:?}", block));

    match slack_client
        .post_message("Manual start", channel_id, token, Some(blocks))
        .await
    {
        Ok(_) => log::info!("Successfully sent test blocks"),
        Err(e) => log::error!("Error sending test blocks: {}", e),
    }
    Ok(())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
