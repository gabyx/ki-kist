// Include all modules.
mod handlers;
mod state;

use std::sync::Arc;

use common::{
    config::get_env_var,
    db,
    log::{create_logger, info, Logger},
};

use handlers::install_handlers;
use state::AppState;

use dotenvy::dotenv;
use rocket::{
    config::{Config, LogLevel},
    Ignite, Rocket,
};

async fn create_rocket(
    log: Arc<Logger>,
    database_url: &str,
) -> Result<Rocket<Ignite>, rocket::Error> {
    let db_conn = db::connect(&log, database_url);
    let app_state = AppState::new(log.clone(), db_conn);

    info!(log, "Start rocket.");

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    let rocket = rocket::custom(config)
        .attach(common::rocket::LogFairing(log))
        .attach(common::rocket::GuardInternalErrors())
        .manage(app_state);

    let rocket = install_handlers(rocket).ignite().await?;

    return Ok(rocket);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let log = create_logger(true);
    info!(log, "Configuring 'API' service.");

    info!(log, "Load environment variables.");
    dotenv().ok();

    let database_url = get_env_var("DATABASE_URL").take();

    let rocket = create_rocket(log.clone(), &database_url).await?;
    rocket.launch().await?;

    info!(&log, "Application terminated.");
    return Ok(());
}
