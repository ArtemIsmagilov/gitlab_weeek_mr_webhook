use actix_web::{App, HttpServer};
use env_logger::Env;

use loggers::create_base_logger;
use services::{healthcheck, index};

pub mod client;
pub mod constants;
pub mod loggers;
pub mod services;
pub mod structures;
pub mod tasks;
pub mod utils;

use constants::{APP_HOST, APP_PORT, APP_WORKERS};

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(create_base_logger())
            .service(index)
            .service(healthcheck)
    })
    .bind((APP_HOST, APP_PORT.parse().unwrap()))?
    .workers(APP_WORKERS.parse().unwrap())
    .run()
    .await
}
