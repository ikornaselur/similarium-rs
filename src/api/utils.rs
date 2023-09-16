use crate::SimilariumError;
use chrono::NaiveTime;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Help,
    ManualStart,
    ManualEnd,
    Debug,
    Start(NaiveTime),
    Stop,
}

pub fn parse_command(text: &str) -> Result<Command, SimilariumError> {
    match text.split_once(' ').unwrap_or((text, "")) {
        ("help", _) => Ok(Command::Help),
        ("start", time) => {
            if time.is_empty() {
                validation_error!(":no_entry_sign: You must specify a time to start the game every day in a 24-hour HH:MM format")
            } else {
                match chrono::NaiveTime::parse_from_str(time, "%H:%M") {
                    Ok(time) => Ok(Command::Start(time)),
                    Err(_) => {
                        validation_error!(":no_entry_sign: Unable to parse the time, please specify it in a 24-hour HH:MM format")
                    }
                }
            }
        }
        ("stop", _) => Ok(Command::Stop),
        ("manual", "start") => Ok(Command::ManualStart),
        ("manual", "end") => Ok(Command::ManualEnd),
        ("debug", _) => Ok(Command::Debug),
        (first, rest) if !rest.is_empty() => {
            validation_error!("Unknown command: {first} {rest}")
        }
        (first, _) => validation_error!("Unknown command: {first}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::SimilariumErrorType;

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
            parse_command("foobar"),
            validation_error!("Unknown command: foobar")
        );
        assert_eq!(
            parse_command("foo bar"),
            validation_error!("Unknown command: foo bar")
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
            parse_command("start 25:00"),
            validation_error!(":no_entry_sign: Unable to parse the time, please specify it in a 24-hour HH:MM format")
        );
        assert_eq!(
            parse_command("start around midnight maybe?"),
            validation_error!(":no_entry_sign: Unable to parse the time, please specify it in a 24-hour HH:MM format"),
        );
    }

    #[test]
    fn test_parse_command_start_parses_time_correctly() {
        assert_eq!(
            parse_command("start 23:59").unwrap(),
            Command::Start(NaiveTime::from_hms_opt(23, 59, 0).unwrap())
        );

        assert_eq!(
            parse_command("start 3:00").unwrap(),
            Command::Start(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
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
            parse_command("manual foobar"),
            validation_error!("Unknown command: manual foobar")
        );
    }
}
