use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum SimilariumErrorType {
    NotFound,
    SlackApiError,
    JsonParseError,
    DbError,
    EnvError,
    IOError,
    Error,
}
#[derive(Debug)]
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
            SimilariumErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::EnvError => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::Error => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::IOError => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::JsonParseError => StatusCode::INTERNAL_SERVER_ERROR,
            SimilariumErrorType::NotFound => StatusCode::NOT_FOUND,
            SimilariumErrorType::SlackApiError => StatusCode::INTERNAL_SERVER_ERROR,
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

impl From<awc::error::SendRequestError> for SimilariumError {
    fn from(error: awc::error::SendRequestError) -> Self {
        log::error!("Error sending request: {}", error);
        SimilariumError {
            message: Some("Unexpected error sending request".to_string()),
            error_type: SimilariumErrorType::SlackApiError,
        }
    }
}

impl From<awc::error::JsonPayloadError> for SimilariumError {
    fn from(error: awc::error::JsonPayloadError) -> Self {
        log::error!("Error parsing JSON: {}", error);
        SimilariumError {
            message: Some("Unexpected error parsing JSON".to_string()),
            error_type: SimilariumErrorType::JsonParseError,
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