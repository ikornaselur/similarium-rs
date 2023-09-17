use crate::SimilariumError;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub slack_client_id: String,
    pub slack_client_secret: String,
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub worker_count: u32,
    pub worker_max_pool_size: u32,
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_WORKER_COUNT: u32 = 3;
const DEFAULT_MAX_POOL_SIZE: u32 = 3;

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init_from_env() -> Result<Self, SimilariumError> {
        let slack_client_id = env::var("SLACK_CLIENT_ID")?;
        let slack_client_secret = env::var("SLACK_CLIENT_SECRET")?;
        let database_url = env::var("DATABASE_URL")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse::<u16>()?;
        let host = env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
        let worker_count = env::var("WORKER_COUNT")
            .unwrap_or_else(|_| DEFAULT_WORKER_COUNT.to_string())
            .parse::<u32>()?;
        let worker_max_pool_size = env::var("WORKER_MAX_POOL_SIZE")
            .unwrap_or_else(|_| DEFAULT_MAX_POOL_SIZE.to_string())
            .parse::<u32>()?;

        Ok(Config {
            slack_client_id,
            slack_client_secret,
            database_url,
            host,
            port,
            worker_count,
            worker_max_pool_size,
        })
    }
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().expect("Failed to initialize config"))
}
