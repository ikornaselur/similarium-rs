#[macro_use]
mod macros;
mod api;
mod error;
mod game;
mod models;
mod payloads;
mod slack_client;
pub mod utils;

pub use api::app::run;
pub use error::{SimilariumError, SimilariumErrorType};
pub use game::{SecretPicker, TARGET_WORDS};
