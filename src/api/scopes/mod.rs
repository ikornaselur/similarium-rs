mod auth;

use actix_web::{get, web, Error, HttpResponse};

#[get("")]
async fn home_handler() -> Result<HttpResponse, Error> {
    log::debug!("GET /");

    Ok(HttpResponse::Ok().body("Home, sweet home!"))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/").service(home_handler);

    conf.service(scope);
}
