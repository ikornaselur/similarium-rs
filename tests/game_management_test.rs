use async_trait::async_trait;
use mockall::mock;
use similarium::{
    game::schedule_game_on_channel,
    payloads::CommandPayload,
    slack_client::{responses::UserInfoResponse, Block, SlackMessage, SlackUserDetails},
    SimilariumError, SimilariumErrorType,
};

fn get_base_command_payload() -> CommandPayload {
    CommandPayload {
        team_id: "invalid_team_id".to_string(),
        channel_id: "invalid_channel_id".to_string(),
        user_id: "invalid_user_id".to_string(),
        text: "invalid_text".to_string(),
        api_app_id: "invalid_api_app_id".to_string(),
    }
}

mock! {
    SlackClient {}

    #[async_trait]
    impl SlackMessage for SlackClient {
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
    impl SlackUserDetails for SlackClient {
        async fn get_user_details(
            &self,
            _user_id: &str,
            _token: &str,
        ) -> Result<UserInfoResponse, SimilariumError>;
    }
}

#[sqlx::test(fixtures("channel"))]
async fn test_schedule_game_on_channel_raises_error_if_channel_already_active(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let now = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let mut payload = get_base_command_payload();
    // Set the channel_id to a valid active channel from the fixtures
    payload.channel_id = "channel_id".to_string();

    let mock_slack_client = MockSlackClient::new();

    let payload = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    assert!(payload.is_err());
    // Assert error being a ValidationError
    let err = payload.unwrap_err();
    assert_eq!(err.error_type, SimilariumErrorType::ValidationError);
    assert!(err.message.is_some());
    assert!(err
        .message
        .unwrap()
        .starts_with(":no_entry_sign: Game is already registered"));

    Ok(())
}
