use similarium::models::{Game, Guess, Word2Vec};
use uuid::Uuid;

#[sqlx::test(fixtures("channel", "game", "user", "words"))]
fn test_adding_guess_to_game(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();

    let guess_count = game.get_guess_count(&pool).await?;
    assert_eq!(guess_count, 0);

    let guess = "apple";
    let secret = Word2Vec {
        word: game.secret.clone(),
    };
    secret.create_materialised_view(&pool).await?;

    let similarity = secret.get_similarity(guess, &pool).await?;

    let guess = Guess {
        id: Uuid::new_v4(),
        game_id,
        updated: 0,
        user_id: "user_id".to_string(),
        word: guess.to_string(),
        rank: similarity.rank,
        similarity: similarity.similarity,
        guess_num: None,
        latest_guess_user_id: "user_id".to_string(),
    };
    guess.insert(&pool).await?;

    let guess_count = game.get_guess_count(&pool).await?;
    assert_eq!(guess_count, 1);

    Ok(())
}
