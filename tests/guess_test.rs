use similarium::game::submit_guess;
use similarium::models::{Game, User, Word2Vec};
use similarium::SimilariumError;
use uuid::Uuid;

#[sqlx::test(fixtures("channel", "games", "users", "words"))]
fn test_submitting_guess_to_game_preprocesses_guesses(
    pool: sqlx::PgPool,
) -> Result<(), SimilariumError> {
    let game_id: Uuid = Uuid::parse_str("00000001-0000-4000-a000-000000000000").unwrap();
    let game = Game::get_by_id(game_id, &pool).await?.unwrap();

    let user = User::get("user_id_1", &pool).await?.unwrap();

    let secret = Word2Vec {
        word: game.secret.clone(),
    };
    secret.create_materialised_view(&pool).await?;

    let guess = submit_guess(&user, &game, "fruit", &pool).await?;
    assert_eq!(guess.word, "fruit");

    let guess = submit_guess(&user, &game, "Happy", &pool).await?;
    assert_eq!(guess.word, "happy");

    let guess = submit_guess(&user, &game, " PEAR ", &pool).await?;
    assert_eq!(guess.word, "pear");

    Ok(())
}
