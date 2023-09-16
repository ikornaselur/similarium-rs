mod game_management;
mod guess;
mod target_words;
pub mod utils;

pub use game_management::{manual_start, start_game, stop_game};
pub use guess::submit_guess;
pub use target_words::TARGET_WORDS;
