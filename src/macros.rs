macro_rules! validation_error {
    ($message:expr) => {
        Err($crate::error::SimilariumError {
            message: Some($message.to_string()),
            error_type: $crate::error::SimilariumErrorType::ValidationError,
        })
    };
    ($message:expr, $($arg:tt)*) => {
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
    ($message:expr, $($arg:tt)*) => {
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
    ($message:expr, $($arg:tt)*) => {
        Err($crate::error::SimilariumError {
            message: Some(format!($message, $($arg)*)),
            error_type: $crate::error::SimilariumErrorType::SlackApiError,
        })
    };
}
