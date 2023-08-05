use crate::models::slack_bot::get_slack_bot_token;
use crate::{SimilariumError, SimilariumErrorType};
use sqlx::postgres::PgPool;

const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";

pub struct SlackClient {
    client: awc::Client,
    token: String,
    channel_id: String,
}

impl SlackClient {
    pub async fn new(
        team_id: String,
        api_app_id: String,
        channel_id: String,
        db: &PgPool,
    ) -> Result<Self, sqlx::Error> {
        let token = get_slack_bot_token(&team_id, &api_app_id, db).await?;

        Ok(Self {
            client: awc::Client::default(),
            token,
            channel_id,
        })
    }

    pub async fn post_message(&self, text: String) -> Result<(), SimilariumError> {
        let res = self
            .client
            .post(POST_MESSAGE_URL)
            .send_form(&[
                ("token", &self.token),
                ("channel", &self.channel_id),
                ("text", &text),
            ])
            .await?;

        log::info!("Slack API response: {:?}", res);

        if res.status().is_success() {
            Ok(())
        } else {
            log::error!("Error posting to Slack API: {}", text);
            Err(SimilariumError {
                message: Some(format!("Error posting to Slack API: {}", text)),
                error_type: SimilariumErrorType::SlackApiError,
            })
        }
    }
}
