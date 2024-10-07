use dotenv::dotenv;
use env_logger;
use std::env;
use log::LevelFilter;
// ... existing code ...

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize the logger
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Start the API server
    api::run_server().await;
}
