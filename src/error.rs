use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum SimilariumErrorType {
    NotFound,
    SlackApiError,
    JsonParseError,
    DbError,
    EnvError,
    IOError,
    Error,
    ValidationError,
    MissingThreadTs,
    SerialisationError,
    ValueError,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SimilariumError {
    pub message: Option<String>,
    pub error_type: SimilariumErrorType,
}

#[derive(Serialize)]
pub struct SimilariumErrorResponse {
    pub error: String,
}

impl fmt::Display for SimilariumErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for SimilariumErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

impl fmt::Display for SimilariumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimilariumError: {:?}", self.error_type)
    }
}

impl ResponseError for SimilariumError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            SimilariumErrorType::DbError
            | SimilariumErrorType::EnvError
            | SimilariumErrorType::Error
            | SimilariumErrorType::IOError
            | SimilariumErrorType::JsonParseError
            | SimilariumErrorType::MissingThreadTs
            | SimilariumErrorType::SerialisationError
            | SimilariumErrorType::ValueError
            | SimilariumErrorType::SlackApiError => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::NotFound => StatusCode::NOT_FOUND,
            SimilariumErrorType::ValidationError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(SimilariumErrorResponse {
            error: self.message(),
        })
    }
}

impl SimilariumError {
    fn message(&self) -> String {
        match self {
            SimilariumError {
                message: Some(message),
                error_type: _,
            } => message.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<SimilariumError> for fang::FangError {
    fn from(error: SimilariumError) -> Self {
        log::error!("Background task error encountered");
        fang::FangError {
            description: error.message(),
        }
    }
}

impl From<sqlx::Error> for SimilariumError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => SimilariumError {
                message: Some("Not found".to_string()),
                error_type: SimilariumErrorType::NotFound,
            },
            _ => {
                log::error!("Database error: {}", error);
                SimilariumError {
                    message: Some("Unexpected database error".to_string()),
                    error_type: SimilariumErrorType::DbError,
                }
            }
        }
    }
}

impl From<sqlx::migrate::MigrateError> for SimilariumError {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        log::error!("Database migration error: {}", error);
        SimilariumError {
            message: Some("Unexpected database migration error".to_string()),
            error_type: SimilariumErrorType::DbError,
        }
    }
}

impl From<reqwest::Error> for SimilariumError {
    fn from(error: reqwest::Error) -> Self {
        log::error!("Error sending request: {}", error);
        SimilariumError {
            message: Some("Unexpected error sending request".to_string()),
            error_type: SimilariumErrorType::SlackApiError,
        }
    }
}

impl From<serde_json::error::Error> for SimilariumError {
    fn from(error: serde_json::error::Error) -> Self {
        log::error!("Error parsing JSON: {}", error);
        SimilariumError {
            message: Some("Unexpected error parsing JSON".to_string()),
            error_type: SimilariumErrorType::JsonParseError,
        }
    }
}

impl From<serde_urlencoded::ser::Error> for SimilariumError {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        log::error!("Error parsing URL encoded data: {}", error);
        SimilariumError {
            message: Some("Unexpected error parsing URL encoded data".to_string()),
            error_type: SimilariumErrorType::SerialisationError,
        }
    }
}

impl From<std::env::VarError> for SimilariumError {
    fn from(error: std::env::VarError) -> Self {
        log::error!("Error parsing environment variable: {}", error);
        SimilariumError {
            message: Some("Unexpected error parsing environment variable".to_string()),
            error_type: SimilariumErrorType::EnvError,
        }
    }
}

impl From<std::num::ParseIntError> for SimilariumError {
    fn from(error: std::num::ParseIntError) -> Self {
        log::error!("Error parsing integer: {}", error);
        SimilariumError {
            message: Some("Unexpected error parsing integer".to_string()),
            error_type: SimilariumErrorType::Error,
        }
    }
}

impl From<std::io::Error> for SimilariumError {
    fn from(error: std::io::Error) -> Self {
        log::error!("IO Error: {}", error);
        SimilariumError {
            message: Some("Unexpected IO error".to_string()),
            error_type: SimilariumErrorType::IOError,
        }
    }
}

impl From<fang::AsyncQueueError> for SimilariumError {
    fn from(error: fang::AsyncQueueError) -> Self {
        log::error!("AsyncQueueError: {}", error);
        SimilariumError {
            message: Some("Unexpected AsyncQueueError".to_string()),
            error_type: SimilariumErrorType::Error,
        }
    }
}
