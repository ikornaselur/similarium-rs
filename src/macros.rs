macro_rules! validation_error {
    ($message:expr) => {
        Err($crate::error::SimilariumError {
            message: Some($message.to_string()),
            error_type: $crate::error::SimilariumErrorType::ValidationError,
        })
    };
    ($message:expr, $($arg:tt)+) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($message, $($arg)*)),
            error_type: $crate::error::SimilariumErrorType::ValidationError,
        })
    };
}

macro_rules! value_error {
    ($message:expr) => {
        Err($crate::error::SimilariumError {
            message: Some($message.to_string()),
            error_type: $crate::error::SimilariumErrorType::ValueError,
        })
    };
    ($message:expr, $($arg:tt)+) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($message, $($arg)*)),
            error_type: $crate::error::SimilariumErrorType::ValueError,
        })
    };
}

macro_rules! slack_api_error {
    ($message:expr) => {
        Err($crate::error::SimilariumError {
            message: Some($message.to_string()),
            error_type: $crate::error::SimilariumErrorType::SlackApiError,
        })
    };
    ($message:expr, $($arg:tt)+) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($message, $($arg)*)),
            error_type: $crate::error::SimilariumErrorType::SlackApiError,
        })
    };
}

#[allow(unused_macros)] // Used in tests
macro_rules! datetime {
    ($year:expr, $month:expr, $day:expr) => {
        chrono::NaiveDate::from_ymd_opt($year, $month, $day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    };
    ($year:expr, $month:expr, $day:expr, $hour:expr, $minute:expr, $second:expr) => {
        chrono::NaiveDate::from_ymd_opt($year, $month, $day)
            .unwrap()
            .and_hms_opt($hour, $minute, $second)
            .unwrap()
            .and_utc()
    };
}

#[cfg(test)]
mod test {
    use crate::{SimilariumError, SimilariumErrorType};

    #[test]
    fn test_validation_error_with_no_arguments() {
        let err: Result<usize, SimilariumError> = validation_error!("test");
        assert!(err.is_err());

        let err = err.unwrap_err();
        assert_eq!(err.error_type, SimilariumErrorType::ValidationError);
        assert_eq!(err.message.unwrap(), "test");
    }

    #[test]
    fn test_validation_error_with_arguments() {
        let err: Result<usize, SimilariumError> = validation_error!("test {} {}", 1, 2);
        assert!(err.is_err());

        let err = err.unwrap_err();
        assert_eq!(err.error_type, SimilariumErrorType::ValidationError);
        assert_eq!(err.message.unwrap(), "test 1 2");
    }
}
