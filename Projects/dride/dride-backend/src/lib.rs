pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod jobs;
pub mod models;
pub mod services;
pub mod ws;

use config::AppConfig;
use ws::hub::Hub;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub redis: redis::Client,
    pub config: AppConfig,
    pub hub: Hub,
}
