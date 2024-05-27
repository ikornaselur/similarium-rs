macro_rules! validation_error {
    ($($t:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($($t)*)),
            error_type: $crate::error::SimilariumErrorType::ValidationError,
        })
    };
}

macro_rules! db_error {
    ($($t:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($($t)*)),
            error_type: $crate::error::SimilariumErrorType::DbError,
        })
    };
}

macro_rules! value_error {
    ($($t:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($($t)*)),
            error_type: $crate::error::SimilariumErrorType::ValueError,
        })
    };
}

macro_rules! slack_api_error {
    ($($t:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($($t)*)),
            error_type: $crate::error::SimilariumErrorType::SlackApiError,
        })
    };
}

macro_rules! ai_error {
    ($($t:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($($t)*)),
            error_type: $crate::error::SimilariumErrorType::AIError,
        })
    };
}

#[cfg(test)]
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
macro_rules! time {
    ($hour:expr, $minute:expr) => {
        chrono::NaiveTime::from_hms_opt($hour, $minute, 0).unwrap()
    };
    ($hour:expr, $minute:expr, $second:expr) => {
        chrono::NaiveTime::from_hms_opt($hour, $minute, $second).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use crate::{SimilariumError, SimilariumErrorType};

    #[test]
    fn test_validation_error_with_no_arguments() {
        let err: Result<usize, SimilariumError> = validation_error!("test");

        assert_eq!(
            err,
            Err(SimilariumError {
                message: Some("test".to_string()),
                error_type: SimilariumErrorType::ValidationError,
            })
        );
    }

    #[test]
    fn test_validation_error_with_arguments() {
        let err: Result<usize, SimilariumError> = validation_error!("test {} {}", 1, 2);

        assert_eq!(
            err,
            Err(SimilariumError {
                message: Some("test 1 2".to_string()),
                error_type: SimilariumErrorType::ValidationError,
            })
        );
    }
}
