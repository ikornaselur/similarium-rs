mod api;
mod error;
mod game;
mod models;
mod payloads;
mod slack_client;

pub use api::app::run;
pub use error::{SimilariumError, SimilariumErrorType};
