pub mod app;
pub mod display;
pub mod widgets;

use crate::cache::Cache;
use crate::models::Config;

pub fn run(config: Config, cache: Cache) {
  let (width, height) = display::get_display_resolution();

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([width as f32, height as f32])
      .with_fullscreen(true)
      .with_decorations(false),
    ..Default::default()
  };

  let _ = eframe::run_native(
    "Healthcheck Dashboard",
    options,
    Box::new(|_cc| {
      Ok(Box::new(app::DashboardApp::new(config, cache)))
    }),
  );
}
