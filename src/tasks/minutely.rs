use crate::{
    db::get_pool,
    game::{end_game, get_active_games_on_channel, start_game_on_channel},
    models::{Channel, SlackBot},
    slack_client::{SlackClient, SlackMessage},
};
use chrono::Timelike;
use fang::{
    async_trait,
    asynk::async_queue::AsyncQueueable,
    serde::{Deserialize, Serialize},
    typetag, AsyncRunnable, FangError, Scheduled,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "fang::serde")]
pub struct GameTask;

#[typetag::serde]
#[async_trait]
impl AsyncRunnable for GameTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        log::debug!("Running GameTask");
        let pool = get_pool();
        let now = chrono::Utc::now();

        let channels = Channel::get_channels_for_hour_minute(
            now.time().hour() as i32,
            now.time().minute() as i32,
            pool,
        )
        .await?;
        if channels.is_empty() {
            return Ok(());
        }

        let slack_client = SlackClient::new();

        // TODO: Shift each of these into a separate task? Should be better for error handling as
        // well and not blocking this task that runs every minute
        for channel in channels {
            // Check if there are any active games on the channel, and end them
            let token = SlackBot::get_slack_bot_token(&channel.team_id, pool).await?;

            let active_games = get_active_games_on_channel(pool, &channel.id).await?;
            let mut should_start_game = true;

            for mut game in active_games {
                if game.get_guess_count(pool).await? == 0 {
                    log::info!("Game with no guesses, not starting a new one");
                    should_start_game = false;
                    slack_client
                        .post_message(
                            "The previous game has no guesses, so I won't start a new one!",
                            &channel.id,
                            &token,
                            None,
                        )
                        .await?;
                    continue;
                }
                end_game(pool, &slack_client, &mut game, &token).await?;
            }
            if should_start_game {
                start_game_on_channel(pool, &slack_client, &channel.id, &token).await?;
            }
        }

        Ok(())
    }

    fn uniq(&self) -> bool {
        true
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "0 * * * * *";
        Some(Scheduled::CronPattern(expression.to_string()))
    }

    fn backoff(&self, attempt: u32) -> u32 {
        u32::pow(2, attempt)
    }
}
