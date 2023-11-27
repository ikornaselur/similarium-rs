use crate::{
    api::app::AppState,
    game::{submit_guess, utils::get_game_blocks},
    models::{Channel, Game, GuessContextOrder, SlackBot, User},
    slack_client::SlackMessage,
    SimilariumError,
};
use actix_web::{get, post, web, Result as AwResult, Scope};
use maud::{html, Markup};
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;
use uuid::Uuid;

#[get("")]
async fn get_index() -> AwResult<Markup> {
    Ok(html! {
        html {
            head {
                script src="https://unpkg.com/htmx.org@1.9.9" {}
                style { (include_str!("../../static/style.css")) }
            }
            body {
                h1 { "Similarium" }
                div #game-state
                    hx-get="/ui/game/XXX"
                    hx-swap="innerHTML"
                    hx-trigger="load";
            }
        }
    })
}

/// TODO: Auth!

async fn get_game_state(game: &Game, db: &sqlx::PgPool) -> Result<Markup, SimilariumError> {
    let latest_game_guesses = game
        .get_guess_contexts(GuessContextOrder::GuessUpdated, 3, db)
        .await?;

    let top_game_guesses = game
        .get_guess_contexts(GuessContextOrder::Rank, 15, db)
        .await?;

    Ok(html! {
        div {
            h2 #puzzle-number { "Puzzle number #" (game.puzzle_number) }
            h3 { "Latest guesses" }
            ul {
                @for guess in &latest_game_guesses {
                    li .guess-context {
                        span .rank {(guess.rank.to_formatted_string(&Locale::en))}
                        span .word {(guess.word)}
                    }
                }
            }
            h3 { "Top guesses" }
            ul {
                @for guess in &top_game_guesses {
                    li .guess-context {
                        span .rank {(guess.rank.to_formatted_string(&Locale::en))}
                        span .word {(guess.word)}
                    }
                }
            }
            form hx-post="/ui/guess" hx-target="#game-state" {
                input #guess-input
                    name="word"
                    type="text"
                    placeholder="Guess...";
                input #guess-game-id
                    name="game_id"
                    type="hidden"
                    value=(game.id);
                input #guess-user-id
                    name="user_id"
                    type="hidden"
                    value="XXX";  // TODO: Should come from session
            }
        }
    })
}

#[get("game/{channel_id}")]
async fn get_game(path: web::Path<String>, app_state: web::Data<AppState>) -> AwResult<Markup> {
    let channel_id: String = path.into_inner();
    let channel = Channel::get(&channel_id, &app_state.db).await?.unwrap();
    let active_games = channel.get_active_games(&app_state.db).await?;

    let game = active_games.first().unwrap();

    let game_state = get_game_state(game, &app_state.db).await?;

    Ok(game_state)
}

#[derive(Deserialize, Debug)]
struct GuessSubmission {
    word: String,
    game_id: Uuid,
    user_id: String, // TODO: Obviously this should be from auth
}

#[post("guess")]
async fn submit_guess_view(
    app_state: web::Data<AppState>,
    guess: web::Form<GuessSubmission>,
) -> AwResult<Markup> {
    let game = Game::get_by_id(guess.game_id, &app_state.db)
        .await?
        .map_or_else(|| validation_error!("Game not found"), Ok)?;
    log::info!("Game: {:?}", game);

    let user = User::get(&guess.user_id, &app_state.db).await?.unwrap();
    log::info!("User: {:?}", user);

    let guess = match submit_guess(&user, &game, &guess.word, &app_state).await {
        Ok(guess) => guess,
        Err(SimilariumError {
            error_type: crate::error::SimilariumErrorType::NotFound,
            ..
        }) => panic!("Word not found in dictionary"),
        Err(e) => panic!("{}", e),
    };
    log::info!("Guess: {:?}", guess);

    // Neet to update game on Slack as well
    let blocks = get_game_blocks(&game, &app_state.db).await?;
    let channel = Channel::get(&game.channel_id, &app_state.db)
        .await?
        .map_or_else(|| validation_error!("Channel not found"), Ok)?;
    let token = SlackBot::get_slack_bot_token(&channel.team_id, &app_state.db).await?;

    let _ = &app_state
        .slack_client
        .chat_update(
            "Update to today's game",
            &game.channel_id,
            &game.thread_ts.clone().unwrap(),
            &token,
            Some(blocks),
        )
        .await?;

    let game_state = get_game_state(&game, &app_state.db).await?;

    Ok(game_state)
}

pub fn scope() -> Scope {
    web::scope("/ui")
        .service(get_index)
        .service(get_game)
        .service(submit_guess_view)
}
