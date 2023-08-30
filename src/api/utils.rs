use crate::SimilariumError;
use time::{macros::format_description, Time};

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    ManualStart,
    ManualEnd,
    Start(Time),
    Stop,
    Invalid(String),
}

pub fn parse_command(text: &str) -> Result<Command, SimilariumError> {
    Ok(match text.split_once(' ').unwrap_or((text, "")) {
        ("help", _) => Command::Help,
        ("start", time) => {
            if time.is_empty() {
                return validation_error!("You must specify a time to start the game every day");
            }
            match Time::parse(time, &format_description!("[hour]:[minute]")) {
                Ok(time) => Command::Start(time),
                Err(_) => Command::Invalid(format!("Invalid time: {time}")),
            }
        }
        ("stop", _) => Command::Stop,
        ("manual", "start") => Command::ManualStart,
        ("manual", "end") => Command::ManualEnd,
        (first, rest) if !rest.is_empty() => {
            Command::Invalid(format!("Unknown command: {first} {rest}"))
        }
        (first, _) => Command::Invalid(format!("Unknown command: {first}")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::SimilariumErrorType;
    use time::macros::time;

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
            parse_command("foobar").unwrap(),
            Command::Invalid("Unknown command: foobar".to_string())
        );
        assert_eq!(
            parse_command("foo bar").unwrap(),
            Command::Invalid("Unknown command: foo bar".to_string())
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
            parse_command("start 25:00").unwrap(),
            Command::Invalid("Invalid time: 25:00".to_string())
        );
        assert_eq!(
            parse_command("start around midnight maybe?").unwrap(),
            Command::Invalid("Invalid time: around midnight maybe?".to_string())
        );
    }

    #[test]
    fn test_parse_command_start_parses_time_correctly() {
        assert_eq!(
            parse_command("start 23:59").unwrap(),
            Command::Start(time!(23:59))
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
            parse_command("manual foobar").unwrap(),
            Command::Invalid("Unknown command: manual foobar".to_string())
        );
    }
}
