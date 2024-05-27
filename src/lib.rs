#[macro_use]
mod macros;
pub mod api;
mod config;
mod db;
mod error;
pub mod game;
pub mod models;
pub mod payloads;
pub mod slack_client;
mod spelling;
mod tasks;
pub mod utils;
pub mod workers;
pub mod ai;

pub use error::{SimilariumError, SimilariumErrorType};
pub use game::TARGET_WORDS;
