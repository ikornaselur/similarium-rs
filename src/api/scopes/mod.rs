mod auth;
mod commands;
mod events;

use actix_web::{get, web, Error, HttpResponse};

#[get("")]
async fn home_handler() -> Result<HttpResponse, Error> {
    log::debug!("GET /");

    Ok(HttpResponse::Ok().body("Home, sweet home!"))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let home_scope = web::scope("")
        .service(home_handler)
        .service(auth::scope())
        .service(events::scope())
        .service(commands::scope());

    conf.service(home_scope);
}
