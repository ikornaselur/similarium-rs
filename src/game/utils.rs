use crate::models::{Game, GuessContextOrder};
use crate::slack_client::Block;
use crate::SimilariumError;
use chrono::{DateTime, Utc};

fn get_base_date() -> DateTime<Utc> {
    datetime!(2022, 5, 6, 0, 0, 0)
}

/// Return a puzzle number for today
///
/// The puzzle number is the number of days that have passed since Similarium started, which was
/// the 6th of May 2022
pub fn get_puzzle_number(date: DateTime<Utc>) -> i64 {
    let delta = date - get_base_date();

    delta.num_days()
}
/// Return the date of a puzzle
///
/// The puzzle date is a nicely formatted date for the puzzle number, such as "Sunday November 13"
/// for puzzle 191
pub fn get_puzzle_date(puzzle_number: i64) -> String {
    let base_date = get_base_date() + chrono::Duration::days(puzzle_number);
    base_date.format("%A %B %-d").to_string()
}

/// Generate the header for the puzzle of the day
pub fn get_header_text(date: DateTime<Utc>) -> String {
    let puzzle_number = get_puzzle_number(date);
    let puzzle_date = get_puzzle_date(puzzle_number);
    format!("{puzzle_date} - Puzzle number {puzzle_number}")
}

/// Generate header body of a game for Slack message
pub fn get_header_body(guesses: i64) -> String {
    format!("*Guesses*: {}", guesses)
}

/// Generate the blocks for a game
pub async fn get_game_blocks(game: Game, db: &sqlx::PgPool) -> Result<Vec<Block>, SimilariumError> {
    let header_body = get_header_body(game.get_guess_count(db).await?);

    let mut blocks = vec![
        Block::header(&game.date),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_puzzle_number() {
        // One day after the game started
        let datetime = datetime!(2022, 5, 7);

        // Game start was considered puzzle 0
        assert_eq!(get_puzzle_number(datetime), 1);

        // Way later, to confirm the puzzle number matches the current date as this test was
        // written
        let datetime = datetime!(2022, 11, 13);
        assert_eq!(get_puzzle_number(datetime), 191);
    }

    #[test]
    fn test_get_puzzle_date() {
        assert_eq!(get_puzzle_date(1), String::from("Saturday May 7"));
        assert_eq!(get_puzzle_date(191), String::from("Sunday November 13"));
    }

    #[test]
    fn test_get_header_text() {
        let datetime = datetime!(2022, 5, 7);
        assert_eq!(
            get_header_text(datetime),
            String::from("Saturday May 7 - Puzzle number 1")
        );

        let datetime = datetime!(2022, 11, 13);
        assert_eq!(
            get_header_text(datetime),
            String::from("Sunday November 13 - Puzzle number 191")
        );
    }

    #[test]
    fn test_get_header_body() {
        assert_eq!(get_header_body(123), String::from("*Guesses*: 123"));
    }
}
