use crate::game::TARGET_WORDS;
use crate::models::{Game, GuessContextOrder};
use crate::slack_client::Block;
use crate::SimilariumError;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

/// Generate the header for the puzzle of the day
pub fn get_header_text(date: DateTime<Utc>, puzzle_number: i64) -> String {
    let puzzle_date = date.format("%A %B %-d").to_string();
    format!("{puzzle_date} - Puzzle number {puzzle_number}")
}

/// Generate header body of a game for Slack message
pub fn get_header_body(guesses: i64) -> String {
    format!("*Guesses*: {}", guesses)
}

/// Generate the blocks for a game
pub async fn get_game_blocks(game: Game, db: &sqlx::PgPool) -> Result<Vec<Block>, SimilariumError> {
    let header_body = get_header_body(game.get_guess_count(db).await?);

    let header = get_header_text(game.date, game.puzzle_number);
    let mut blocks = vec![
        Block::header(&header),
        Block::section(&header_body),
        // TODO: If finished?
    ];

    // Show latest
    if game.active {
        blocks.push(Block::section("*Latest guesses*"));

        let game_guesses = game
            .get_guess_contexts(GuessContextOrder::GuessNum, 3, db)
            .await?;
        blocks.extend(
            game_guesses
                .into_iter()
                .map(|guess| Block::guess_context("latest", guess)),
        );
    }

    // Show top
    blocks.push(Block::section("*Top guesses*"));
    let game_guesses = game
        .get_guess_contexts(GuessContextOrder::Rank, 15, db)
        .await?;
    blocks.extend(
        game_guesses
            .into_iter()
            .map(|guess| Block::guess_context("top", guess)),
    );

    // Show input
    if game.active {
        blocks.push(Block::guess_input());
    }

    Ok(blocks)
}

/// Get the secret word for a channel and puzzle number
///
/// The channel_id is used as a random seed, then the puzzle number is used to pick the randomly
/// sorted target words list.
pub fn get_secret(seed: String, puzzle_number: i64) -> String {
    let mut rng: Pcg64 = Seeder::from(seed).make_rng();

    // Get a copy of the target words
    let mut target_words = TARGET_WORDS.to_vec();

    // Shuffle..
    target_words.shuffle(&mut rng);

    // Then return the puzzle_number-th word, making sure to wrap around
    target_words[puzzle_number as usize % target_words.len()].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_header_text() {
        let datetime = datetime!(2022, 5, 7);
        let puzzle_number = 123;
        assert_eq!(
            get_header_text(datetime, puzzle_number),
            String::from("Saturday May 7 - Puzzle number 123")
        );
    }

    #[test]
    fn test_get_header_body() {
        assert_eq!(get_header_body(123), String::from("*Guesses*: 123"));
    }

    #[test]
    fn test_get_secret_is_consistent() {
        let seed = "foobar".to_string();

        let secret1 = get_secret(seed.clone(), 0);
        let secret2 = get_secret(seed.clone(), 0);

        assert_eq!(secret1, secret2);
    }

    #[test]
    fn test_get_secret_gives_different_values_for_different_seeds() {
        let seed1 = "foobar".to_string();
        let seed2 = "bazqux".to_string();

        let secret1 = get_secret(seed1.clone(), 0);
        let secret2 = get_secret(seed2.clone(), 0);

        assert_ne!(secret1, secret2);
    }

    #[test]
    fn test_get_secret_wraps_around() {
        let total_target_words = TARGET_WORDS.len();
        let seed = "foobar".to_string();

        let secret1 = get_secret(seed.clone(), 0);
        let secret2 = get_secret(seed.clone(), total_target_words as i64);

        assert_eq!(secret1, secret2);
    }
}
