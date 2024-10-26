mod api_service;
mod app_context;
pub mod macros;
mod models;
mod vector_store;
mod web_server;
use tracing::{event, span, Level};
use web_server::run_actix_server;
pub(crate) mod api;
pub mod config_loader;
pub mod cosql;
pub mod distance;
pub mod quantization;
pub mod storage;
pub mod indexes;

use crate::models::common::*;

fn main() -> Result<(), std::io::Error> {
    // TODO: Load tracing configuration to config.toml, or from command line arguments
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::TRACE)
        .init();
    let x = span!(Level::TRACE, "Cosdata");
    let _x = x.enter();
    event!(Level::INFO, "Database started");
    run_actix_server()
        .inspect(|_| event!(Level::INFO, "Database stopped succesfully"))
        .inspect_err(|e| {
            event!(Level::ERROR, "Database stopped, becauseof error {:#?}", e);
        })
}
