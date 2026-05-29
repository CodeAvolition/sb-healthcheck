mod cache;
mod checker;
mod models;
mod server;
mod ui;

use cache::{create_cache, start_polling_loop};
use models::Config;
use server::start_server;
use std::fs;

fn main() {
  // Handle --version flag
  let args: Vec<String> = std::env::args().collect();
  if args.len() > 1 && args[1] == "--version" {
    println!("v{}", env!("CARGO_PKG_VERSION"));
    return;
  }

  let config_str = fs::read_to_string("config.json")
    .expect("Failed to read config.json");
  let config: Config = serde_json::from_str(&config_str)
    .expect("Failed to parse config.json");

  println!("Loaded project: {}", config.project_name);

  let cache = create_cache();

  // Spawn tokio runtime in background thread
  // This handles both polling AND the axum web server
  let config_clone = config.clone();
  let cache_clone = cache.clone();
  std::thread::spawn(move || {
    tokio::runtime::Runtime::new().unwrap().block_on(
      async move {
        let cache_for_poll = cache_clone.clone();
        let config_for_poll = config_clone.clone();

        tokio::spawn(async move {
          start_polling_loop(
            config_for_poll,
            cache_for_poll,
          )
          .await;
        });

        start_server(config_clone, cache_clone).await;
      },
    );
  });

  // egui runs on main thread
  // On Pi: only start if HDMI is connected
  // On dev machine: always start
  let should_start_ui = if cfg!(target_arch = "arm") {
    ui::display::is_hdmi_connected()
  } else {
    true
  };

  if should_start_ui {
    ui::run(config, cache);
  } else {
    println!("No HDMI detected, running server only");
    // Block main thread
    loop {
      std::thread::sleep(std::time::Duration::from_secs(
        60,
      ));
    }
  }
}
