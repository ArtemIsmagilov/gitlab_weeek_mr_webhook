use actix_web::middleware::Logger;

pub fn create_base_logger() -> Logger {
    Logger::default()
}
