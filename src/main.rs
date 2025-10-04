mod cache;
mod checker;
mod models;
mod server;

use cache::{create_cache, start_polling_loop};
use models::Config;
use server::start_server;
use std::fs;

#[tokio::main]
async fn main() {
  let config_str = fs::read_to_string("config.json")
    .expect("Failed to read config.json");

  let config: Config = serde_json::from_str(&config_str)
    .expect("Failed to parse config.json");

  println!("Loaded project: {}", config.project_name);

  let cache = create_cache();

  // Spawn background polling
  let config_clone = config.clone();
  let cache_clone = cache.clone();
  tokio::spawn(async move {
    start_polling_loop(config_clone, cache_clone).await;
  });

  // Start web server
  start_server(config, cache).await;
}
