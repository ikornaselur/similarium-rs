mod channel;
mod game;
mod slack_bot;
mod word2vec;

// Expose the models directly
pub use channel::Channel;
pub use game::Game;
pub use slack_bot::SlackBot;
pub use word2vec::{Similarity, Word2Vec};
