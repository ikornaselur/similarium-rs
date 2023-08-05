use crate::slack_client::responses::SlackOAuthResponse;
use crate::{SimilariumError, SimilariumErrorType};

const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";
const OAUTH_API_URL: &str = "https://slack.com/api/oauth.v2.access";

pub struct SlackClient {
    client: awc::Client,
}

impl SlackClient {
    pub fn new() -> Self {
        Self {
            client: awc::Client::default(),
        }
    }

    pub async fn post_message(
        &self,
        text: &str,
        channel_id: &str,
        token: &str,
    ) -> Result<(), SimilariumError> {
        let res = self
            .client
            .post(POST_MESSAGE_URL)
            .send_form(&[("token", token), ("channel", channel_id), ("text", text)])
            .await?;

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

    pub async fn post_oauth_code(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<SlackOAuthResponse, SimilariumError> {
        let mut res = self
            .client
            .post(OAUTH_API_URL)
            .send_form(&[
                ("code", code),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ])
            .await?;

        Ok(res.json::<SlackOAuthResponse>().await?)
    }
}
