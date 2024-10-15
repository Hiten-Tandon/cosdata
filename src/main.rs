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

use crate::models::common::*;

fn main() -> Result<(), std::io::Error> {
    let span = span!(Level::TRACE, "Database");
    let _x = span.enter();
    event!(Level::INFO, "Database started");
    run_actix_server()
}
