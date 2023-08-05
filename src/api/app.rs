use actix_web::{error, web, App, HttpRequest, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::api::config::Config;
use crate::api::scopes;
use crate::SimilariumError;

async fn not_found(request: HttpRequest, text: String) -> HttpResponse {
    log::error!("404: {} {}", request.method(), request.path());
    log::debug!("Headers: {:?}", request.headers());
    log::debug!("Body: {}", text);

    HttpResponse::NotFound().body("Not Found")
}

pub(crate) struct AppState {
    pub db: sqlx::PgPool,
    pub config: Config,
}

pub async fn run() -> Result<(), SimilariumError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(":info"));

    log::info!("Running migrations");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, req| {
            log::error!("JSON error: {}", err);
            log::info!("{:?}", req);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        });

    log::info!("Starting server on {host}:{port}");

    let slack_client_id = env::var("SLACK_CLIENT_ID")?;
    let slack_client_secret = env::var("SLACK_CLIENT_SECRET")?;

    HttpServer::new(move || {
        App::new()
            .app_data(json_cfg.clone())
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                config: Config {
                    slack_client_id: slack_client_id.clone(),
                    slack_client_secret: slack_client_secret.clone(),
                },
            }))
            .configure(scopes::config)
            .default_service(web::get().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
