use crate::{
    models::{SlackBot, User},
    slack_client::{responses::UserInfoResponse, SlackClient},
    SimilariumError,
};
use chrono::{NaiveTime, Timelike};

pub async fn get_or_create_user(
    user_id: &str,
    team_id: &str,
    db: &sqlx::PgPool,
    slack_client: &SlackClient,
) -> Result<User, SimilariumError> {
    let user = match User::get(user_id, db).await? {
        Some(user) => user,
        None => {
            log::debug!("Creating user");

            // Fetch the user details from Slack
            let token = SlackBot::get_slack_bot_token(team_id, db).await?;
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

/// Convert a naive time to a human readable time
pub fn when_human(time: NaiveTime) -> String {
    let when_fmt = time.format("%H:%M").to_string();
    match time.hour() {
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

/// Convert a NaiveTime and a timezone_offset into a UTC NaiveTime
pub fn get_utc_naive_time(time: NaiveTime, timezone_offset: i32) -> NaiveTime {
    time - chrono::Duration::seconds(timezone_offset as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_human() {
        assert_eq!(when_human(time!(0, 0)), "late night at 00:00");
        assert_eq!(when_human(time!(1, 0)), "late night at 01:00");
        assert_eq!(when_human(time!(2, 0)), "late night at 02:00");
        assert_eq!(when_human(time!(3, 0)), "late night at 03:00");
        assert_eq!(when_human(time!(4, 0)), "in the early morning at 04:00");
        assert_eq!(when_human(time!(5, 0)), "in the early morning at 05:00");
        assert_eq!(when_human(time!(6, 0)), "in the early morning at 06:00");
        assert_eq!(when_human(time!(7, 0)), "in the early morning at 07:00");
        assert_eq!(when_human(time!(8, 0)), "in the morning at 08:00");
        assert_eq!(when_human(time!(9, 0)), "in the morning at 09:00");
        assert_eq!(when_human(time!(10, 0)), "in the morning at 10:00");
        assert_eq!(when_human(time!(11, 0)), "in the morning at 11:00");
        assert_eq!(when_human(time!(12, 0)), "at noon at 12:00");
        assert_eq!(when_human(time!(13, 0)), "in the afternoon at 13:00");
        assert_eq!(when_human(time!(14, 0)), "in the afternoon at 14:00");
        assert_eq!(when_human(time!(15, 0)), "in the afternoon at 15:00");
        assert_eq!(when_human(time!(16, 0)), "in the afternoon at 16:00");
        assert_eq!(when_human(time!(17, 0)), "in the evening at 17:00");
        assert_eq!(when_human(time!(18, 0)), "in the evening at 18:00");
        assert_eq!(when_human(time!(19, 0)), "in the evening at 19:00");
        assert_eq!(when_human(time!(20, 0)), "in the evening at 20:00");
        assert_eq!(when_human(time!(21, 0)), "at night at 21:00");
        assert_eq!(when_human(time!(22, 0)), "at night at 22:00");
        assert_eq!(when_human(time!(23, 0)), "at night at 23:00");

        assert_eq!(when_human(time!(12, 34)), "at noon at 12:34");
    }

    #[test]
    fn test_get_utc_naive_time_without_offset() {
        assert_eq!(get_utc_naive_time(time!(0, 0), 0), time!(0, 0));
        assert_eq!(get_utc_naive_time(time!(11, 0), 0), time!(11, 0));
    }

    #[test]
    fn test_get_utc_naive_time_with_positive_offset() {
        assert_eq!(get_utc_naive_time(time!(0, 0), 3600), time!(23, 0));
        assert_eq!(get_utc_naive_time(time!(0, 0), 1800), time!(23, 30));
    }

    #[test]
    fn test_get_utc_naive_time_with_negative_offset() {
        assert_eq!(get_utc_naive_time(time!(0, 0), -3600), time!(1, 0));
        assert_eq!(get_utc_naive_time(time!(0, 0), -1800), time!(0, 30));
    }
}
