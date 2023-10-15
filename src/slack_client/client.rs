use crate::{
    slack_client::{
        responses::{SlackOAuthResponse, UserInfoResponse},
        traits::{SlackMessage, SlackOAuth, SlackUserDetails},
        Block,
    },
    SimilariumError,
};
use async_trait::async_trait;

const CHAT_UPDATE_URL: &str = "https://slack.com/api/chat.update";
const OAUTH_API_URL: &str = "https://slack.com/api/oauth.v2.access";
const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";
const USER_DETAILS_URL: &str = "https://slack.com/api/users.info";
const POST_EPHEMERAL_URL: &str = "https://slack.com/api/chat.postEphemeral";

pub struct SlackClient {
    client: reqwest::Client,
}

impl SlackClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for SlackClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SlackMessage for SlackClient {
    async fn post_message(
        &self,
        text: &str,
        channel_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError> {
        let res = if let Some(blocks) = blocks {
            self.client
                .post(POST_MESSAGE_URL)
                .form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("text", text),
                    ("blocks", &serde_json::to_string(&blocks).unwrap()),
                ])
                .send()
                .await?
        } else {
            self.client
                .post(POST_MESSAGE_URL)
                .form(&[("token", token), ("channel", channel_id), ("text", text)])
                .send()
                .await?
        };

        if !res.status().is_success() {
            log::error!("Error posting to Slack API: {}", text);
            return slack_api_error!("Error posting to Slack API: {}", text);
        }

        let payload = res.json::<serde_json::Value>().await?;
        let ok = payload["ok"].as_bool().unwrap_or(false);
        if !ok {
            log::error!("Error posting to Slack API: {}", payload);
            return slack_api_error!("Error posting to Slack API: {}", payload);
        }

        Ok(payload)
    }

    async fn post_ephemeral(
        &self,
        text: &str,
        channel_id: &str,
        user_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError> {
        let res = if let Some(blocks) = blocks {
            self.client
                .post(POST_EPHEMERAL_URL)
                .form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("text", text),
                    ("user", user_id),
                    ("blocks", &serde_json::to_string(&blocks).unwrap()),
                ])
                .send()
                .await?
        } else {
            self.client
                .post(POST_EPHEMERAL_URL)
                .form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("text", text),
                    ("user", user_id),
                ])
                .send()
                .await?
        };

        if !res.status().is_success() {
            log::error!("Error posting to Slack API: {}", text);
            return slack_api_error!("Error posting to Slack API: {}", text);
        }

        let payload = res.json::<serde_json::Value>().await?;
        let ok = payload["ok"].as_bool().unwrap_or(false);
        if !ok {
            log::error!("Error posting to Slack API: {}", payload);
            return slack_api_error!("Error posting to Slack API: {}", payload);
        }

        Ok(payload)
    }

    async fn chat_update(
        &self,
        text: &str,
        channel_id: &str,
        message_ts: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError> {
        let res = if let Some(blocks) = blocks {
            self.client
                .post(CHAT_UPDATE_URL)
                .form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("ts", message_ts),
                    ("text", text),
                    ("blocks", &serde_json::to_string(&blocks).unwrap()),
                ])
                .send()
                .await?
        } else {
            self.client
                .post(POST_MESSAGE_URL)
                .form(&[
                    ("token", token),
                    ("channel", channel_id),
                    ("ts", message_ts),
                    ("text", text),
                ])
                .send()
                .await?
        };

        if !res.status().is_success() {
            log::error!("Error posting to Slack API: {}", text);
            return slack_api_error!("Error posting to Slack API: {}", text);
        }

        let payload = res.json::<serde_json::Value>().await?;
        let ok = payload["ok"].as_bool().unwrap_or(false);
        if !ok {
            log::error!("Error posting to Slack API: {}", payload);
            return slack_api_error!("Error posting to Slack API: {}", payload);
        }

        Ok(payload)
    }
}

#[async_trait]
impl SlackOAuth for SlackClient {
    async fn post_oauth_code(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<SlackOAuthResponse, SimilariumError> {
        let res = self
            .client
            .post(OAUTH_API_URL)
            .form(&[
                ("code", code),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ])
            .send()
            .await?;

        Ok(res.json::<SlackOAuthResponse>().await?)
    }
}

#[async_trait]
impl SlackUserDetails for SlackClient {
    async fn get_user_details(
        &self,
        user_id: &str,
        token: &str,
    ) -> Result<UserInfoResponse, SimilariumError> {
        let res = self
            .client
            .get(USER_DETAILS_URL)
            .query(&[("user", user_id)])
            .bearer_auth(token)
            .send()
            .await?;

        Ok(res.json::<UserInfoResponse>().await?)
    }
}
