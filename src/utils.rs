use crate::models::{SlackBot, User};
use crate::slack_client::responses::UserInfoResponse;
use crate::slack_client::SlackClient;
use crate::SimilariumError;
use chrono::{NaiveTime, Timelike};

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
                    return validation_error!("Error fetching user details: {:?}", error);
                }
                _ => {
                    return validation_error!("Error fetching user details");
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

/// Convert an hour to a human readable time
pub fn when_human(hour: u32) -> String {
    let when_fmt = format!("{:02}:00", hour);
    match hour {
        0..=3 => format!("late night at {}", when_fmt),
        4..=7 => format!("in the early morning at {}", when_fmt),
        8..=11 => format!("in the morning at {}", when_fmt),
        12 => format!("at noon at {}", when_fmt),
        13..=16 => format!("in the afternoon at {}", when_fmt),
        17..=20 => format!("in the evening at {}", when_fmt),
        21..=23 => format!("at night at {}", when_fmt),
        _ => format!("at {}", when_fmt),
    }
}

/// Take a NaiveTime and a timezone offset in seconds and return the hour
pub fn get_hour(time: NaiveTime, timezone_offset: i32) -> i32 {
    let hours_offset = timezone_offset / 3600;
    let utc_hour = time.hour() as i32 - hours_offset;
    (utc_hour + 24) %  24
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_human() {
        assert_eq!(when_human(0), "late night at 00:00");
        assert_eq!(when_human(1), "late night at 01:00");
        assert_eq!(when_human(2), "late night at 02:00");
        assert_eq!(when_human(3), "late night at 03:00");
        assert_eq!(when_human(4), "in the early morning at 04:00");
        assert_eq!(when_human(5), "in the early morning at 05:00");
        assert_eq!(when_human(6), "in the early morning at 06:00");
        assert_eq!(when_human(7), "in the early morning at 07:00");
        assert_eq!(when_human(8), "in the morning at 08:00");
        assert_eq!(when_human(9), "in the morning at 09:00");
        assert_eq!(when_human(10), "in the morning at 10:00");
        assert_eq!(when_human(11), "in the morning at 11:00");
        assert_eq!(when_human(12), "at noon at 12:00");
        assert_eq!(when_human(13), "in the afternoon at 13:00");
        assert_eq!(when_human(14), "in the afternoon at 14:00");
        assert_eq!(when_human(15), "in the afternoon at 15:00");
        assert_eq!(when_human(16), "in the afternoon at 16:00");
        assert_eq!(when_human(17), "in the evening at 17:00");
        assert_eq!(when_human(18), "in the evening at 18:00");
        assert_eq!(when_human(19), "in the evening at 19:00");
        assert_eq!(when_human(20), "in the evening at 20:00");
        assert_eq!(when_human(21), "at night at 21:00");
        assert_eq!(when_human(22), "at night at 22:00");
        assert_eq!(when_human(23), "at night at 23:00");
    }

    #[test]
    fn test_get_hour_with_utc_offset() {
        assert_eq!(get_hour(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), 0), 0);
        assert_eq!(get_hour(NaiveTime::from_hms_opt(13, 0, 0).unwrap(), 0), 13);
        assert_eq!(get_hour(NaiveTime::from_hms_opt(23, 0, 0).unwrap(), 0), 23);
    }

    #[test]
    fn test_get_hour_with_plus_1_hour_offset() {
        assert_eq!(
            get_hour(NaiveTime::from_hms_opt(0, 0, 0).unwrap(), 3600),
            23
        );
        assert_eq!(
            get_hour(NaiveTime::from_hms_opt(12, 0, 0).unwrap(), 3600),
            11
        );
    }
}
