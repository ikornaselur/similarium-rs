use crate::models::{Game, GuessContextOrder};
use crate::slack_client::Block;
use crate::SimilariumError;
use time::{
    macros::{datetime, format_description},
    OffsetDateTime,
};

const BASE_DATE: OffsetDateTime = datetime!(2022-05-06 00:00:00 UTC);

/// Return a puzzle number for today
///
/// The puzzle number is the number of days that have passed since Similarium started, which was
/// the 6th of May 2022
pub fn get_puzzle_number(date: OffsetDateTime) -> i64 {
    let delta = date - BASE_DATE;

    delta.whole_days()
}
/// Return the date of a puzzle
///
/// The puzzle date is a nicely formatted date for the puzzle number, such as "Sunday November 13"
/// for puzzle 191
pub fn get_puzzle_date(puzzle_number: i64) -> Result<String, time::Error> {
    let base_date = BASE_DATE + time::Duration::days(puzzle_number);
    let format = format_description!("[weekday] [month repr:long] [day padding:none]");

    Ok(base_date.format(&format)?.to_string())
}

/// Generate the header for the puzzle of the day
pub fn get_header_text(date: OffsetDateTime) -> Result<String, time::Error> {
    let puzzle_number = get_puzzle_number(date);
    let puzzle_date = get_puzzle_date(puzzle_number)?;
    Ok(format!("{puzzle_date} - Puzzle number {puzzle_number}"))
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
        let datetime = datetime!(2022-05-07 00:00:00 UTC);

        // Game start was considered puzzle 0
        assert_eq!(get_puzzle_number(datetime), 1);

        // Way later, to confirm the puzzle number matches the current date as this test was
        // written
        let datetime = datetime!(2022-11-13 00:00:00 UTC);
        assert_eq!(get_puzzle_number(datetime), 191);
    }

    #[test]
    fn test_get_puzzle_date() {
        assert_eq!(get_puzzle_date(1).unwrap(), String::from("Saturday May 7"));
        assert_eq!(
            get_puzzle_date(191).unwrap(),
            String::from("Sunday November 13")
        );
    }

    #[test]
    fn test_get_header_text() {
        let datetime = datetime!(2022-05-07 00:00:00 UTC);
        assert_eq!(
            get_header_text(datetime).unwrap(),
            String::from("Saturday May 7 - Puzzle number 1")
        );

        let datetime = datetime!(2022-11-13 00:00:00 UTC);
        assert_eq!(
            get_header_text(datetime).unwrap(),
            String::from("Sunday November 13 - Puzzle number 191")
        );
    }

    #[test]
    fn test_get_header_body() {
        assert_eq!(get_header_body(123), String::from("*Guesses*: 123"));
    }
}
