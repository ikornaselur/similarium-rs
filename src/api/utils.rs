use crate::{SimilariumError, SimilariumErrorType};
use chrono::NaiveTime;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Help,
    ManualStart,
    ManualEnd,
    Start(NaiveTime),
    Stop,
}

fn parse_command(text: &str) -> Result<Command, SimilariumError> {
    let (first, rest) = text.split_once(' ').unwrap_or((text, ""));

    Ok(match (first, rest) {
        ("help", _) => Command::Help,
        ("start", time) => {
            if time.is_empty() {
                Err(SimilariumError {
                    message: Some(
                        "You must specify a time to start the game every day".to_string(),
                    ),
                    error_type: SimilariumErrorType::ValidationError,
                })?;
            }
            Command::Start(NaiveTime::parse_from_str(time, "%H:%M")?)
        }
        ("stop", _) => Command::Stop,
        ("manual", "start") => Command::ManualStart,
        ("manual", "end") => Command::ManualEnd,
        (_, _) => Err(SimilariumError {
            message: Some(format!("Unknown command: {}", first)),
            error_type: SimilariumErrorType::ValidationError,
        })?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("help").unwrap(), Command::Help);
    }

    #[test]
    fn test_parse_command_handles_spaces() {
        assert_eq!(parse_command("help me please").unwrap(), Command::Help);
    }

    #[test]
    fn test_parse_command_returns_error_on_unknown_command() {
        assert_eq!(
            parse_command("foobar").unwrap_err().error_type,
            SimilariumErrorType::ValidationError
        );
    }

    #[test]
    fn test_parse_command_start_raises_if_no_time_given() {
        assert_eq!(
            parse_command("start").unwrap_err().error_type,
            SimilariumErrorType::ValidationError
        );
    }

    #[test]
    fn test_parse_command_start_raises_with_invalid_time() {
        assert_eq!(
            parse_command("start 25:00").unwrap_err().error_type,
            SimilariumErrorType::ValidationError
        );
        assert_eq!(
            parse_command("start around midnight maybe?")
                .unwrap_err()
                .error_type,
            SimilariumErrorType::ValidationError
        );
    }

    #[test]
    fn test_parse_command_start_parses_time_correctly() {
        assert_eq!(
            parse_command("start 23:59").unwrap(),
            Command::Start(NaiveTime::from_hms_opt(23, 59, 0).unwrap())
        );
    }

    #[test]
    fn test_parse_command_manual_start() {
        assert_eq!(parse_command("manual start").unwrap(), Command::ManualStart);
    }

    #[test]
    fn test_parse_command_manual_end() {
        assert_eq!(parse_command("manual end").unwrap(), Command::ManualEnd);
    }

    #[test]
    fn test_parse_command_manual_unknown() {
        assert_eq!(
            parse_command("manual foobar").unwrap_err().error_type,
            SimilariumErrorType::ValidationError
        );
    }
}
