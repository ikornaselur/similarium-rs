#[macro_use]
mod macros;
pub mod api;
mod error;
mod game;
mod models;
mod payloads;
mod slack_client;
mod tasks;
pub mod utils;
pub mod workers;

pub use error::{SimilariumError, SimilariumErrorType};
pub use game::TARGET_WORDS;
