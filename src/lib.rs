#[macro_use]
mod macros;
pub mod api;
mod config;
mod db;
mod error;
mod game;
pub mod models;
mod payloads;
mod slack_client;
mod tasks;
pub mod utils;
pub mod workers;

pub use error::{SimilariumError, SimilariumErrorType};
pub use game::TARGET_WORDS;
