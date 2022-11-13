use chrono::Utc;
use sqlx::sqlite::SqlitePool;

use crate::game::utils::{get_puzzle_date, get_puzzle_number};
use crate::models::Game;

#[allow(dead_code)]
async fn start_game(pool: &SqlitePool, channel_id: String) -> anyhow::Result<Game> {
    let puzzle_number = get_puzzle_number(Utc::now());
    let puzzle_date = get_puzzle_date(puzzle_number);

    let game = Game::new(
        pool,
        channel_id,
        String::from(""),
        puzzle_number,
        puzzle_date,
        true,
    )
    .await?;

    Ok(game)
}
