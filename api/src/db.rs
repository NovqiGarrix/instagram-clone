use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::{info, instrument};
use crate::configuration::{Settings};

#[instrument(name = "Connect to PostgresSQL", skip(config))]
pub async fn connect_db(config: &Settings) -> Result<DatabaseConnection, DbErr> {
    let db_url = if config.application.rust_env == "testing" {
        info!("Testing Mode -> Using TEST_DATABASE_URL");
        let tdb = config
            .test_database.as_ref().expect("No test database config");
        tdb.connection_string()
    } else {
        info!("-> Using DATABASE_URL");
        config
            .database.connection_string()
    };

    let mut db_options = ConnectOptions::new(db_url);

    if config.application.rust_env == "testing" {
        db_options.sqlx_logging(true);
    } else {
        db_options
            .max_connections(50)
            .min_connections(3)
            .connect_timeout(Duration::from_secs(30))
            .acquire_timeout(Duration::from_secs(60))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(60 * 5))
            .sqlx_logging(&config.application.rust_env == "development");
    }

    info!("Connecting to database...");
    let db = Database::connect(db_options).await?;

    Ok(db)
}