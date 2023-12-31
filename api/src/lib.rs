use sea_orm::DatabaseConnection;
use crate::configuration::Settings;
use crate::error::HttpResponseError;

pub mod configuration;
pub mod app;
pub mod db;
pub mod utils;
pub mod error;

// ----- Domain -----
pub mod auth;
pub mod logging;
mod routes;
// ----- End Domain -----

#[derive(Clone)]
pub struct AppState {
    pub config: Settings,
    pub db: DatabaseConnection
}

pub type Result<T> = std::result::Result<T, HttpResponseError>;
