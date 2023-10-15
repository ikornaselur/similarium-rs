use similarium::game::utils::get_header_body;
use similarium::models::Game;
use similarium::SimilariumError;
use uuid::Uuid;

#[sqlx::test(fixtures("channel", "games"))]
fn test_get_header_body_active_game(pool: sqlx::PgPool) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();

    let header_body = get_header_body(&game, &pool).await;

    let expected = ["*Guesses*: 0"].join("\n");

    assert_eq!(header_body, expected);

    Ok(())
}

#[sqlx::test(fixtures("channel", "games", "users", "words", "guesses"))]
fn test_get_header_body_active_game_with_guesses(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();

    let header_body = get_header_body(&game, &pool).await;

    let expected = ["*Guesses*: 2"].join("\n");

    assert_eq!(header_body, expected);

    Ok(())
}

#[sqlx::test(fixtures("channel", "games"))]
fn test_get_header_body_inactive_game_no_winners(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000002-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();

    let header_body = get_header_body(&game, &pool).await;

    let expected = ["The secret was *secret* :tada:", "*No winners*"].join("\n");

    assert_eq!(header_body, expected);

    Ok(())
}

#[sqlx::test(fixtures("channel", "games", "users", "guesses"))]
fn test_get_header_body_inactive_game_with_winners(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000002-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();
    game.add_winner("user_id_1", 1, &pool).await?;

    let header_body = get_header_body(&game, &pool).await;

    let expected = [
        "The secret was *secret* :tada:",
        "*Winners*",
        ":first_place_medal: <@user_id_1> on guess 1",
    ]
    .join("\n");

    assert_eq!(header_body, expected);

    Ok(())
}
