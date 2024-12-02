use camera_rest_api_lib::run_server;
use log::{debug, info};
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let default_level = "debug".to_string();
    let log_level = env::var("RUST_LOG").unwrap_or(default_level);
    env_logger::Builder::new()
        .parse_filters(&log_level)
        .init();

    info!("Запуск сервера...");
    run_server("config.yml").await
}
