use crate::{
    game::TARGET_WORDS,
    models::{Game, GuessContextOrder},
    slack_client::Block,
    SimilariumError,
};
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

/// Generate the header for the puzzle of the day
pub fn get_header_text(date: DateTime<Utc>, puzzle_number: i64) -> String {
    let puzzle_date = date.format("%A %B %-d").to_string();
    format!("{puzzle_date} - Puzzle number {puzzle_number}")
}

fn get_medal(idx: usize) -> String {
    match idx {
        0 => ":first_place_medal:",
        1 => ":second_place_medal:",
        2 => ":third_place_medal:",
        _ => "",
    }
    .to_string()
}

/// Generate header body of a game for Slack message
///
/// If the game is active, the guess count is shown
/// If there are any winners, they are always shown
/// If the game is not active, the secret is shown
pub async fn get_header_body(game: &Game, db: &sqlx::PgPool) -> String {
    let guesses = game.get_guess_count(db).await.unwrap_or(0);
    let winners = game.get_winners(db).await.unwrap_or(vec![]);

    let mut lines: Vec<String> = vec![];

    if game.active {
        lines.push(format!("*Guesses*: {}", guesses));
    } else {
        lines.push(format!("The secret was *{}* :tada:", game.secret));
    }

    if !winners.is_empty() {
        lines.push("*Winners*".to_string());
        for (idx, winner) in winners.iter().enumerate() {
            lines.push(format!(
                "{} <@{}> on guess {}",
                get_medal(idx),
                winner.user_id,
                winner.guess_idx
            ));
        }
        lines.join("\n");
    } else if !game.active {
        lines.push("*No winners*".to_string());
    }

    lines.join("\n")
}

/// Generate the blocks for a game
pub async fn get_game_blocks(
    game: &Game,
    db: &sqlx::PgPool,
) -> Result<Vec<Block>, SimilariumError> {
    let header_body = get_header_body(game, db).await;
    let header = get_header_text(game.date, game.puzzle_number);
    let guess_count = game.get_guess_count(db).await.unwrap_or(0);

    let mut blocks = vec![Block::header(&header), Block::section(&header_body, None)];

    if guess_count > 0 {
        // Show latest, only while game is active
        if game.active {
            blocks.push(Block::section("*Latest guesses*", None));

            let game_guesses = game
                .get_guess_contexts(GuessContextOrder::GuessUpdated, 3, db)
                .await?;
            blocks.extend(
                game_guesses
                    .into_iter()
                    .map(|guess| Block::guess_context("latest", guess, game.active)),
            );
        }

        // Show top, if there are any so far
        blocks.push(Block::section("*Top guesses*", None));
        let game_guesses = game
            .get_guess_contexts(GuessContextOrder::Rank, 15, db)
            .await?;
        blocks.extend(
            game_guesses
                .into_iter()
                .map(|guess| Block::guess_context("top", guess, game.active)),
        );
    }

    // Show input
    if game.active {
        blocks.push(Block::guess_input());
    }

    Ok(blocks)
}

/// Generate the help messsage for the user
pub fn get_help_blocks() -> Vec<Block> {
    vec![
        Block::section("Hello there :wave: here's what you can do!", None),
        Block::section(
            "*Start a daily puzzle at a specific time*\nStart posting a \
            daily puzzle at the provided time on the current channel. The \
            time has to be in 24-hour format, like \"13:00\" for example.\nThe \
            time will be based on your timezone.\nThe puzzle will be \
            posted at the start of the hour.\n_Please note that the daily \
            game is posted at the start of every hour, so if you specify \
            13:45, it will be posted at 13:00_",
            Some(vec!["Start a daily puzzle", "`/similarium start [time]`"]),
        ),
        Block::section(
            "*Stop posting the daily puzzle*\nStop posting a daily puzzle \
            if there is one",
            Some(vec!["Stop a daily puzzle", "`/similarium stop`"]),
        ),
        Block::section(
            "*About*",
            Some(vec![
                "Version",
                option_env!("PACKAGE_VERSION").unwrap_or("unknown"),
            ]),
        ),
    ]
}

/// Get the secret word for a channel and puzzle number
///
/// The channel_id is used as a random seed, then the puzzle number is used to pick the randomly
/// sorted target words list.
pub fn get_secret(seed: &str, puzzle_number: i64) -> String {
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
    fn test_get_secret_is_consistent() {
        let seed = "foobar";

        let secret1 = get_secret(seed, 0);
        let secret2 = get_secret(seed, 0);

        assert_eq!(secret1, secret2);
    }

    #[test]
    fn test_get_secret_gives_different_values_for_different_seeds() {
        let seed1 = "foobar";
        let seed2 = "bazqux";

        let secret1 = get_secret(seed1, 0);
        let secret2 = get_secret(seed2, 0);

        assert_ne!(secret1, secret2);
    }

    #[test]
    fn test_get_secret_wraps_around() {
        let total_target_words = TARGET_WORDS.len();
        let seed = "foobar";

        let secret1 = get_secret(seed, 0);
        let secret2 = get_secret(seed, total_target_words as i64);

        assert_eq!(secret1, secret2);
    }

    #[test]
    fn test_get_medal() {
        assert_eq!(get_medal(0), ":first_place_medal:");
        assert_eq!(get_medal(1), ":second_place_medal:");
        assert_eq!(get_medal(2), ":third_place_medal:");
        assert_eq!(get_medal(3), "");
    }
}
