use similarium::models::{Game, Guess, GuessContextOrder, Word2Vec};
use similarium::SimilariumError;
use uuid::Uuid;

#[sqlx::test(fixtures("channel", "game", "users", "words"))]
fn test_adding_guess_to_game(pool: sqlx::PgPool) -> Result<(), SimilariumError> {
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
        user_id: "user_id_1".to_string(),
        word: guess.to_string(),
        rank: similarity.rank,
        similarity: similarity.similarity,
        guess_num: None,
        latest_guess_user_id: "user_id_1".to_string(),
    };
    guess.insert(&pool).await?;

    let guess_count = game.get_guess_count(&pool).await?;
    assert_eq!(guess_count, 1);

    Ok(())
}

#[sqlx::test(fixtures("channel", "game", "users", "words", "guesses"))]
fn test_get_guess_contexts_order_by_updated_gives_latest_guesser(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?;

    assert!(game.is_some());
    let game = game.unwrap();

    let guess_contexts = game
        .get_guess_contexts(GuessContextOrder::GuessUpdated, 3, &pool)
        .await?;
    assert_eq!(guess_contexts.len(), 2);

    // GuessUpdated will return the latest guess first
    assert_eq!(guess_contexts[0].guess_num, 2);
    assert_eq!(guess_contexts[0].word, "happy");
    assert_eq!(guess_contexts[0].username, "user_1");

    assert_eq!(guess_contexts[1].guess_num, 1);
    assert_eq!(guess_contexts[1].word, "fruit");
    assert_eq!(guess_contexts[1].username, "user_3");

    Ok(())
}

#[sqlx::test(fixtures("channel", "game", "users", "words", "guesses"))]
fn test_get_guess_contexts_order_by_rank_gives_original_guesser(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?;

    assert!(game.is_some());
    let game = game.unwrap();

    let guess_contexts = game
        .get_guess_contexts(GuessContextOrder::Rank, 3, &pool)
        .await?;
    assert_eq!(guess_contexts.len(), 2);

    // Rank will return the guesses in order of guessing
    assert_eq!(guess_contexts[0].guess_num, 1);
    assert_eq!(guess_contexts[0].word, "fruit");
    assert_eq!(guess_contexts[0].username, "user_3");

    assert_eq!(guess_contexts[1].guess_num, 2);
    assert_eq!(guess_contexts[1].word, "happy");
    assert_eq!(guess_contexts[1].username, "user_3");

    Ok(())
}
