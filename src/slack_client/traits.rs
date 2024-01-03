use crate::{
    slack_client::{
        responses::{SlackOAuthResponse, UserInfoResponse},
        Block,
    },
    SimilariumError,
};
use std::future::Future;

pub trait SlackMessage {
    fn post_message(
        &self,
        text: &str,
        channel_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> impl Future<Output = Result<serde_json::Value, SimilariumError>>;

    fn post_ephemeral(
        &self,
        text: &str,
        channel_id: &str,
        user_id: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> impl Future<Output = Result<serde_json::Value, SimilariumError>>;

    fn chat_update(
        &self,
        text: &str,
        channel_id: &str,
        message_ts: &str,
        token: &str,
        blocks: Option<Vec<Block>>,
    ) -> impl Future<Output = Result<serde_json::Value, SimilariumError>>;
}

pub trait SlackOAuth {
    fn post_oauth_code(
        &self,
        code: &str,
        client_id: &str,
        client_secret: &str,
    ) -> impl Future<Output = Result<SlackOAuthResponse, SimilariumError>>;
}

pub trait SlackUserDetails {
    fn get_user_details(
        &self,
        user_id: &str,
        token: &str,
    ) -> impl Future<Output = Result<UserInfoResponse, SimilariumError>>;
}
