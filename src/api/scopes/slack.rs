use crate::app::AppState;
use crate::slack_objects::Event;
use actix_web::{post, web, Error, HttpResponse, Scope};
use serde::Deserialize;

const POST_MESSAGE_URL: &str = "https://slack.com/api/chat.postMessage";

#[derive(Deserialize, Debug)]
struct EventPayload {
    team_id: String,
    channel_id: String,
    user_id: String,
    text: String,
    api_app_id: String,
}

#[post("/events")]
async fn post_events(_event: web::Json<Event>) -> Result<HttpResponse, Error> {
    log::debug!("POST /slack/events");

    Ok(HttpResponse::Ok().into())
}

#[post("/commands/{command}")]
async fn post_commands(
    path: web::Path<String>,
    form: web::Form<EventPayload>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let command = path.into_inner();
    let payload = form.into_inner();

    log::debug!("POST /slack/commands/{}", command);

    // Get the bot token
    let token = sqlx::query!(
        r#"
        SELECT
            bot_token
        FROM
            slack_bots
        WHERE
            team_id=$1 AND app_id=$2
        ORDER BY 
            installed_at DESC
        LIMIT 1;
        "#,
        payload.team_id,
        payload.api_app_id,
    )
    .fetch_one(&app_state.db)
    .await
    .unwrap()
    .bot_token
    .unwrap();

    // Post a message, with the text, to the channel using the post message endpoint
    let client = awc::Client::default();
    client
        .post(POST_MESSAGE_URL)
        .send_form(&[
            ("token", token),
            ("channel", payload.channel_id),
            (
                "text",
                format!("Hey <@{}>! You said: {}", payload.user_id, payload.text),
            ),
        ])
        .await
        .unwrap();

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("slack")
        .service(post_events)
        .service(post_commands)
}
