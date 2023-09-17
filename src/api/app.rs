use actix_web::{error, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use fang::{asynk::async_queue::AsyncQueue, NoTls};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    api::{config::Config, scopes},
    slack_client::SlackClient,
    workers::{ensure_recurring_tasks, start_workers},
    SimilariumError,
};

async fn not_found(request: HttpRequest, text: String) -> HttpResponse {
    log::error!("404: {} {}", request.method(), request.path());
    log::debug!("Headers: {:?}", request.headers());
    log::debug!("Body: {}", text);

    HttpResponse::NotFound().body("Not Found")
}

pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Config,
    pub slack_client: SlackClient,
    pub queue: Arc<Mutex<AsyncQueue<NoTls>>>,
}

pub async fn run() -> Result<(), SimilariumError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config::init_from_env()?;

    log::info!("Running migrations");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, req| {
            log::error!("JSON error: {}", err);
            log::info!("{:?}", req);
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        });

    let queue = start_workers(
        &config.database_url,
        config.worker_count,
        config.worker_max_pool_size,
    )
    .await?;

    ensure_recurring_tasks(queue.clone()).await?;

    log::info!("Starting server on {}:{}", config.host, config.port);
    let bind_tuple = (config.host.clone(), config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(json_cfg.clone())
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                config: config.clone(),
                slack_client: SlackClient::new(),
                queue: Arc::new(Mutex::new(queue.clone())),
            }))
            .configure(scopes::config)
            .default_service(web::get().to(not_found))
    })
    .bind(bind_tuple)?
    .run()
    .await?;

    Ok(())
}
