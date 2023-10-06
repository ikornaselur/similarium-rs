use crate::{
    api::AppState,
    models::{Game, Guess, User, Word2Vec},
    spelling::americanise,
    SimilariumError,
};
use uuid::Uuid;

pub async fn submit_guess(
    user: &User,
    game: &Game,
    guess: &str,
    app_state: &AppState,
) -> Result<Guess, SimilariumError> {
    // Get the similarity for the guess
    let secret = Word2Vec {
        word: game.secret.clone(),
    };
    let guess = americanise(guess);
    let similarity = secret.get_similarity(&guess, &app_state.db).await?;

    if let Some(mut guess) = Guess::get(game.id, &guess, &app_state.db).await? {
        log::debug!("Guess has already been made, updating timestamp");
        guess
            .update_latest_guess_user_id(&user.id, &app_state.db)
            .await?;

        return Ok(guess);
    }

    let mut guess = Guess {
        id: Uuid::new_v4(),
        game_id: game.id,
        updated: chrono::Utc::now().timestamp_millis(),
        user_id: user.id.clone(),
        word: guess.to_string(),
        rank: similarity.rank,
        similarity: similarity.similarity,
        guess_num: None,
        latest_guess_user_id: user.id.clone(),
    };
    guess.insert(&app_state.db).await?;
    guess.refresh_guess_num(&app_state.db).await?;

    Ok(guess)
}
