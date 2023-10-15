use async_trait::async_trait;
use mockall::mock;
use mockall::predicate::*;
use similarium::{
    game::schedule_game_on_channel,
    models::Channel,
    payloads::{CommandPayload, Profile, UserInfo},
    slack_client::{responses::UserInfoResponse, Block, SlackMessage, SlackUserDetails},
    SimilariumError, SimilariumErrorType,
};

fn get_test_command_payload() -> CommandPayload {
    CommandPayload {
        team_id: "invalid_team_id".to_string(),
        channel_id: "invalid_channel_id".to_string(),
        user_id: "invalid_user_id".to_string(),
        text: "invalid_text".to_string(),
        api_app_id: "invalid_api_app_id".to_string(),
    }
}

fn get_test_user_info_response() -> UserInfoResponse {
    UserInfoResponse {
        user: Some(UserInfo {
            id: "user_id".to_string(),
            name: "user_name".to_string(),
            real_name: "user_real_name".to_string(),
            profile: Profile {
                avatar_hash: "avatar_hash".to_string(),
                status_text: "status_text".to_string(),
                status_emoji: "status_emoji".to_string(),
                real_name: "real_name".to_string(),
                display_name: "display_name".to_string(),
                real_name_normalized: "real_name_normalized".to_string(),
                display_name_normalized: "display_name_normalized".to_string(),
                image_original: "image_original".to_string(),
                image_24: "image_24".to_string(),
                image_32: "image_32".to_string(),
                image_48: "image_48".to_string(),
                image_72: "image_72".to_string(),
                image_192: "image_192".to_string(),
                image_512: "image_512".to_string(),
            },
            tz: "tz".to_string(),
            tz_offset: 0,
        }),
        ok: true,
        error: None,
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
    let mut payload = get_test_command_payload();
    // Set the channel_id to a valid active channel from the fixtures
    payload.channel_id = "channel_id".to_string();

    let mock_slack_client = MockSlackClient::new();

    let payload = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    assert!(payload.is_err());

    let err = payload.unwrap_err();
    assert_eq!(err.error_type, SimilariumErrorType::ValidationError);
    assert!(err.message.is_some());
    assert!(err
        .message
        .unwrap()
        .starts_with(":no_entry_sign: Game is already registered"));

    Ok(())
}

#[sqlx::test(fixtures("channel"))]
async fn test_schedule_game_on_channel_raises_error_if_unable_to_get_user_details(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let now = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let payload = get_test_command_payload();

    let mut mock_slack_client = MockSlackClient::new();
    mock_slack_client
        .expect_get_user_details()
        .returning(|_, _| {
            Err(SimilariumError {
                error_type: SimilariumErrorType::ValidationError,
                message: Some("Unable to get user details".to_string()),
            })
        });

    let payload = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    assert!(payload.is_err());

    let err = payload.unwrap_err();
    assert_eq!(err.error_type, SimilariumErrorType::SlackApiError);
    assert!(err.message.is_some());
    assert!(err
        .message
        .unwrap()
        .starts_with("Error fetching user details"));

    Ok(())
}

#[sqlx::test(fixtures("channel"))]
async fn test_schedule_game_on_channel_posts_to_slack_that_user_started_game(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let now = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let payload = get_test_command_payload();

    let mut mock_slack_client = MockSlackClient::new();
    mock_slack_client
        .expect_get_user_details()
        .returning(|_, _| Ok(get_test_user_info_response()));
    mock_slack_client
        .expect_post_message()
        .returning(|message, _, _, _| {
            assert_eq!(
                message,
                "<@user_id> has started a daily game of Similarium late night at 00:00 UTC"
            );
            Ok(serde_json::Value::Null)
        });

    let _ = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    Ok(())
}
#[sqlx::test(fixtures("channel"))]
async fn test_schedule_game_on_channel_creates_channel_if_it_doesnt_exist(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let now = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let payload = get_test_command_payload();

    assert!(Channel::get(payload.channel_id.as_str(), &pool)
        .await?
        .is_none());

    let mut mock_slack_client = MockSlackClient::new();
    mock_slack_client
        .expect_get_user_details()
        .returning(|_, _| Ok(get_test_user_info_response()));
    mock_slack_client
        .expect_post_message()
        .returning(|_, _, _, _| Ok(serde_json::Value::Null));

    let _ = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    assert!(Channel::get(payload.channel_id.as_str(), &pool)
        .await?
        .is_some());

    Ok(())
}

#[sqlx::test(fixtures("channel"))]
async fn test_schedule_game_on_channel_updates_channel_if_it_exists(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let now = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let mut payload = get_test_command_payload();
    // Set the channel_id to a valid active channel from the fixtures
    payload.channel_id = "channel_id".to_string();

    let mut existing_channel = Channel::get(payload.channel_id.as_str(), &pool)
        .await?
        .expect("Channel should already exist");
    existing_channel.active = false;
    existing_channel.update(&pool).await?;

    let mut mock_slack_client = MockSlackClient::new();
    mock_slack_client
        .expect_get_user_details()
        .returning(|_, _| Ok(get_test_user_info_response()));
    mock_slack_client
        .expect_post_message()
        .returning(|_, _, _, _| Ok(serde_json::Value::Null));

    let _ = schedule_game_on_channel(&pool, &mock_slack_client, &payload, "token", now).await;

    let existing_channel = Channel::get(payload.channel_id.as_str(), &pool)
        .await?
        .expect("Channel should already exist");
    assert!(existing_channel.active);

    Ok(())
}
