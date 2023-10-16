use similarium::slack_client::{SlackClient, SlackMessage, POST_MESSAGE_PATH};
use similarium::SimilariumError;

#[sqlx::test]
async fn test_slack_client_post_message_sends_request_to_slack() -> Result<(), SimilariumError> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut server = mockito::Server::new();

    // TODO: Add matchers
    let mock = server
        .mock("POST", POST_MESSAGE_PATH)
        .with_status(200)
        .with_body(r#"{"ok": true}"#)
        .create();

    let slack_client = SlackClient::new(server.url());

    let text = "Hello, world!";
    let channel_id = "channel_id";
    let token = "token_x";

    let _ = slack_client
        .post_message(text, channel_id, token, None)
        .await?;

    mock.assert();

    Ok(())
}
