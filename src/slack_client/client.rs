use crate::slack_client::responses::{SlackOAuthResponse, UserInfoResponse};
use crate::slack_client::Block;
use crate::{SimilariumError, SimilariumErrorType};

const OAUTH_API_URL: &str = "https://slack.com/api/oauth.v2.access";
const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";
const USER_DETAILS_URL: &str = "https://slack.com/api/users.info";

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
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError> {
        let mut res = if let Some(blocks) = blocks {
            self.client
                .post(POST_MESSAGE_URL)
                .send_form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("text", text),
                    ("blocks", &serde_json::to_string(&blocks).unwrap()),
                ])
                .await?
        } else {
            self.client
                .post(POST_MESSAGE_URL)
                .send_form(&[("token", token), ("channel", channel_id), ("text", text)])
                .await?
        };

        if !res.status().is_success() {
            log::error!("Error posting to Slack API: {}", text);
            return Err(SimilariumError {
                message: Some(format!("Error posting to Slack API: {}", text)),
                error_type: SimilariumErrorType::SlackApiError,
            });
        }

        let payload = res.json::<serde_json::Value>().await?;
        let ok = payload["ok"].as_bool().unwrap_or(false);
        if !ok {
            log::error!("Error posting to Slack API: {}", payload);
            return Err(SimilariumError {
                message: Some(format!("Error posting to Slack API: {}", payload)),
                error_type: SimilariumErrorType::SlackApiError,
            });
        }

        Ok(payload)
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

    pub async fn get_user_details(
        &self,
        user_id: &str,
        token: &str,
    ) -> Result<UserInfoResponse, SimilariumError> {
        let mut res = self
            .client
            .get(USER_DETAILS_URL)
            .query(&[("user", user_id)])?
            .bearer_auth(token)
            .send()
            .await?;

        Ok(res.json::<UserInfoResponse>().await?)
    }
}

impl Default for SlackClient {
    fn default() -> Self {
        Self::new()
    }
}
