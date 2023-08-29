use crate::models::{SlackBot, User};
use crate::slack_client::responses::UserInfoResponse;
use crate::slack_client::SlackClient;
use crate::SimilariumError;

pub async fn get_or_create_user(
    user_id: &str,
    team_id: &str,
    api_app_id: &str,
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
) -> Result<User, SimilariumError> {
    let user = match User::get(user_id, db).await? {
        Some(user) => user,
        None => {
            log::debug!("Creating user");

            // Fetch the user details from Slack
            let token = SlackBot::get_slack_bot_token(team_id, api_app_id, db).await?;
            let response = &slack_client.get_user_details(user_id, &token).await?;

            let user_details = match response {
                UserInfoResponse {
                    ok: true,
                    user: Some(user),
                    ..
                } => user,
                UserInfoResponse {
                    ok: false,
                    error: Some(error),
                    ..
                } => {
                    return Err(SimilariumError::validation_error(
                        format!("Error fetching user details: {:?}", error).as_str(),
                    ));
                }
                _ => {
                    return Err(SimilariumError::validation_error(
                        "Error fetching user details",
                    ));
                }
            };

            let user = User {
                id: user_details.id.clone(),
                profile_photo: user_details.profile.image_24.clone(),
                username: user_details.name.clone(),
            };
            user.insert(db).await?;
            user
        }
    };

    Ok(user)
}
