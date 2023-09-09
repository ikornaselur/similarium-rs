use crate::api::app::AppState;
use crate::models::{Game, Guess, User, Word2Vec};
use crate::SimilariumError;
use uuid::Uuid;

pub async fn submit_guess(
    user: &User,
    game: &Game,
    guess: &str,
    app_state: &AppState,
) -> Result<(), SimilariumError> {
    // Get the similarity for the guess
    let secret = Word2Vec {
        word: game.secret.clone(),
    };
    let similarity = secret.get_similarity(guess, &app_state.db).await?;

    if Guess::get(game.id, guess, &app_state.db).await?.is_some() {
        // TODO: Implement an update to the game on this
        return validation_error!("You have already guessed this word");
    }

    let guess = Guess {
        id: Uuid::new_v4(),
        game_id: game.id,
        updated: 0,
        user_id: user.id.clone(),
        word: guess.to_string(),
        rank: similarity.rank,
        similarity: similarity.similarity,
        guess_num: None,
        latest_guess_user_id: user.id.clone(),
    };
    guess.insert(&app_state.db).await?;

    log::debug!("Similarity: {:?}", similarity);

    Ok(())
}
