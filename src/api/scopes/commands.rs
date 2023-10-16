use crate::slack_client::SlackMessage;
use actix_web::{post, web, HttpResponse, Scope};

use crate::{
    api::{
        app::AppState,
        utils::{parse_command, Command},
    },
    game::{
        end_game, get_active_games_on_channel, manual_start, schedule_game_on_channel,
        stop_games_on_channel, utils::get_help_blocks,
    },
    models::SlackBot,
    payloads::CommandPayload,
    SimilariumError,
};

#[post("/similarium")]
async fn post_similarium_command(
    form: web::Form<CommandPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SimilariumError> {
    let payload = form.into_inner();
    let token = SlackBot::get_slack_bot_token(&payload.team_id, &app_state.db).await?;
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
        Command::Start(time) => match schedule_game_on_channel(
            &app_state.db,
            &app_state.slack_client,
            &payload,
            &token,
            time,
        )
        .await
        {
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
        Command::Stop => {
            match stop_games_on_channel(&app_state.db, &app_state.slack_client, &payload, &token)
                .await
            {
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
            }
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
        Command::ManualEnd => {
            let active_games =
                get_active_games_on_channel(&app_state.db, &payload.channel_id).await?;
            for mut game in active_games {
                end_game(&app_state.db, &app_state.slack_client, &mut game, &token).await?;
            }
        }
        Command::Debug => todo!(),
    }

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/commands").service(post_similarium_command)
}
