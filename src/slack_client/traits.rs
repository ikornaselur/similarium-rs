use crate::{
    slack_client::{
        responses::{SlackOAuthResponse, UserInfoResponse},
        Block,
    },
    SimilariumError,
};
use async_trait::async_trait;

#[async_trait]
pub trait SlackMessage {
    async fn post_message(
        &self,
        text: &str,
        channel_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError>;

    async fn post_ephemeral(
        &self,
        text: &str,
        channel_id: &str,
        user_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError>;

    async fn chat_update(
        &self,
        text: &str,
        channel_id: &str,
        message_ts: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> Result<serde_json::Value, SimilariumError>;
}

#[async_trait]
pub trait SlackOAuth {
    async fn post_oauth_code(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Result<SlackOAuthResponse, SimilariumError>;
}

#[async_trait]
pub trait SlackUserDetails {
    async fn get_user_details(
        &self,
        user_id: &str,
        token: &str,
    ) -> Result<UserInfoResponse, SimilariumError>;
}
