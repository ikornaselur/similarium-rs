use crate::{
    slack_client::{
        responses::{SlackOAuthResponse, UserInfoResponse},
        traits::{SlackMessage, SlackOAuth, SlackUserDetails},
        Block,
    },
    SimilariumError,
};
use async_trait::async_trait;

const CHAT_UPDATE_PATH: &str = "/chat.update";
const OAUTH_API_PATH: &str = "/oauth.v2.access";
const POST_MESSAGE_PATH: &str = "/chat.postMessage";
const USER_DETAILS_PATH: &str = "/users.info";
const POST_EPHEMERAL_PATH: &str = "/chat.postEphemeral";

pub struct SlackClient {
    client: reqwest::Client,
    base_url: String,
}

impl SlackClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl Default for SlackClient {
    fn default() -> Self {
        Self::new("https://slack.com/api".to_string())
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
                .post(self.get_url(POST_MESSAGE_PATH))
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
                .post(self.get_url(POST_MESSAGE_PATH))
                .form(&[("token", token), ("channel", channel_id), ("text", text)])
                .send()
                .await?
        };

        if !res.status().is_success() {
            let res_text = res.text().await?;
            log::error!("Error posting to Slack API: {}", res_text);
            return slack_api_error!("Error posting to Slack API: {}", res_text);
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
                .post(self.get_url(POST_EPHEMERAL_PATH))
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
                .post(self.get_url(POST_EPHEMERAL_PATH))
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
                .post(self.get_url(CHAT_UPDATE_PATH))
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
                .post(self.get_url(CHAT_UPDATE_PATH))
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
            .post(self.get_url(OAUTH_API_PATH))
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
            .get(self.get_url(USER_DETAILS_PATH))
            .query(&[("user", user_id)])
            .bearer_auth(token)
            .send()
            .await?;

        Ok(res.json::<UserInfoResponse>().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};

    #[actix_web::test]
    async fn test_slack_client_post_message_sends_request_to_slack() -> Result<(), SimilariumError>
    {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";

        let mut server = Server::new();

        let mock = server
            .mock("POST", POST_MESSAGE_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .post_message(text, channel_id, token, None)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_post_message_sends_request_to_slack_with_blocks(
    ) -> Result<(), SimilariumError> {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";
        let blocks = Some(vec![Block::section("Hello, blocks!", None)]);

        let mut server = Server::new();

        let mock = server
            .mock("POST", POST_MESSAGE_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
                Matcher::UrlEncoded("blocks".into(), serde_json::to_string(&blocks).unwrap()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .post_message(text, channel_id, token, blocks)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_post_ephemeral_sends_request_to_slack() -> Result<(), SimilariumError>
    {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";
        let user_id = "user_x";

        let mut server = Server::new();

        let mock = server
            .mock("POST", POST_EPHEMERAL_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
                Matcher::UrlEncoded("user".into(), user_id.into()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .post_ephemeral(text, channel_id, user_id, token, None)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_post_ephemeral_sends_request_to_slack_with_blocks(
    ) -> Result<(), SimilariumError> {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";
        let user_id = "user_x";
        let blocks = Some(vec![Block::section("Hello, blocks!", None)]);

        let mut server = Server::new();

        let mock = server
            .mock("POST", POST_EPHEMERAL_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
                Matcher::UrlEncoded("user".into(), user_id.into()),
                Matcher::UrlEncoded("blocks".into(), serde_json::to_string(&blocks).unwrap()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .post_ephemeral(text, channel_id, user_id, token, blocks)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }
    #[actix_web::test]
    async fn test_slack_client_chat_update_sends_request_to_slack() -> Result<(), SimilariumError> {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";
        let ts = "123456.789012";

        let mut server = Server::new();

        let mock = server
            .mock("POST", CHAT_UPDATE_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
                Matcher::UrlEncoded("ts".into(), ts.into()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .chat_update(text, channel_id, ts, token, None)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_chat_update_sends_request_to_slack_with_blocks(
    ) -> Result<(), SimilariumError> {
        let text = "Hello, world!";
        let channel_id = "channel_x";
        let token = "token_x";
        let ts = "123456.789012";
        let blocks = Some(vec![Block::section("Hello, blocks!", None)]);

        let mut server = Server::new();

        let mock = server
            .mock("POST", CHAT_UPDATE_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("token".into(), token.into()),
                Matcher::UrlEncoded("channel".into(), channel_id.into()),
                Matcher::UrlEncoded("text".into(), text.into()),
                Matcher::UrlEncoded("ts".into(), ts.into()),
                Matcher::UrlEncoded("blocks".into(), serde_json::to_string(&blocks).unwrap()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .chat_update(text, channel_id, ts, token, blocks)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_post_oauth_code_sends_request_to_slack(
    ) -> Result<(), SimilariumError> {
        let code = "code_x";
        let client_id = "client_id_x";
        let client_secret = "client_secret_x";

        let mut server = Server::new();

        let mock = server
            .mock("POST", OAUTH_API_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true, "app_id": "123", "team": {"id": "team_id"}, "is_enterprise_install": false}"#)
            .match_body(Matcher::AllOf(vec![
                Matcher::UrlEncoded("code".into(), code.into()),
                Matcher::UrlEncoded("client_id".into(), client_id.into()),
                Matcher::UrlEncoded("client_secret".into(), client_secret.into()),
            ]))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client
            .post_oauth_code(code, client_id, client_secret)
            .await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }

    #[actix_web::test]
    async fn test_slack_client_get_user_details_sends_request_to_slack(
    ) -> Result<(), SimilariumError> {
        let user_id = "user_x";
        let token = "token_x";

        let mut server = Server::new();

        let mock = server
            .mock("GET", USER_DETAILS_PATH)
            .with_status(200)
            .with_body(r#"{"ok": true}"#)
            .match_query(Matcher::UrlEncoded("user".into(), user_id.into()))
            .match_header("Authorization", Matcher::Exact(format!("Bearer {}", token)))
            .create();

        let slack_client = SlackClient::new(server.url());

        let request = slack_client.get_user_details(user_id, token).await;

        mock.assert();

        assert!(request.is_ok());

        Ok(())
    }
}
