use crate::api::payloads::Event;
use actix_web::{post, web, Error, HttpResponse, Scope};

#[post("/")]
async fn post_events(_event: web::Json<Event>) -> Result<HttpResponse, Error> {
    log::debug!("POST /slack/events");

    Ok(HttpResponse::Ok().into())
}

pub fn scope() -> Scope {
    web::scope("/events").service(post_events)
}
