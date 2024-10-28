use actix_web::{App, HttpServer};
use env_logger::Env;

use loggers::create_base_logger;
use services::index;

pub mod client;
pub mod constants;
pub mod loggers;
pub mod services;
pub mod structures;
pub mod tasks;
pub mod utils;

use constants::{HOST_PORT, WORKERS};

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new().wrap(create_base_logger()).service(index))
        .bind(&*HOST_PORT)?
        .workers(*WORKERS)
        .run()
        .await
}
