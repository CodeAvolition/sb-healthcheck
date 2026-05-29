use crate::cache::Cache;
use crate::models::check::CheckResult;
use crate::models::config::Environment;
use crate::models::{Config, Status};
use eframe::egui;
use egui::{Color32, RichText, Ui};
use std::time::Duration;

// Dark background
const COLOR_BG: Color32 = Color32::from_rgb(26, 26, 26);
const COLOR_CARD: Color32 = Color32::from_rgb(42, 42, 42);
const COLOR_CARD_INNER: Color32 =
  Color32::from_rgb(52, 52, 52);

// Brand
const COLOR_PRIMARY: Color32 =
  Color32::from_rgb(0, 131, 143);
const COLOR_PRIMARY_DARK: Color32 =
  Color32::from_rgb(0, 96, 100);

// Status
const COLOR_SUCCESS: Color32 =
  Color32::from_rgb(76, 175, 80);
const COLOR_UNHEALTHY: Color32 =
  Color32::from_rgb(214, 76, 35);
const COLOR_ERROR: Color32 = Color32::from_rgb(183, 28, 28);

// Text
const COLOR_TEXT: Color32 =
  Color32::from_rgb(236, 239, 241);
const COLOR_TEXT_DIM: Color32 =
  Color32::from_rgb(160, 170, 175);

pub struct DashboardApp {
  config: Config,
  cache: Cache,
}

impl DashboardApp {
  pub fn new(config: Config, cache: Cache) -> Self {
    Self { config, cache }
  }

  fn status_icon_and_text(
    status: &Status,
  ) -> (&'static str, &'static str, Color32) {
    match status {
      Status::Healthy => ("✓", "Healthy", COLOR_SUCCESS),
      Status::Unhealthy => {
        ("✗", "Unhealthy", COLOR_UNHEALTHY)
      }
      Status::Error => ("⚠", "Error", COLOR_ERROR),
    }
  }

  fn render_environment(
    &self,
    ui: &mut Ui,
    env: &Environment,
    available_width: f32,
  ) {
    egui::Frame::default()
      .fill(COLOR_CARD)
      .rounding(egui::Rounding::same(4.0))
      .show(ui, |ui| {
        ui.set_min_width(available_width);

        // Environment header
        egui::Frame::default()
          .fill(COLOR_PRIMARY)
          .inner_margin(egui::Margin::symmetric(10.0, 6.0))
          .rounding(egui::Rounding {
            nw: 4.0,
            ne: 4.0,
            sw: 0.0,
            se: 0.0,
          })
          .show(ui, |ui| {
            ui.set_min_width(available_width);
            ui.label(
              RichText::new(&env.name)
                .color(COLOR_TEXT)
                .strong()
                .size(14.0),
            );
          });

        // Checks grid
        egui::Frame::default()
          .inner_margin(egui::Margin::same(8.0))
          .show(ui, |ui| {
            ui.set_max_width(available_width - 16.0);
            ui.horizontal_wrapped(|ui| {
              ui.spacing_mut().item_spacing =
                egui::vec2(6.0, 6.0);
              for check in &env.checks {
                let key =
                  format!("{}:{}", env.name, check.name);
                if let Some(result) = self.cache.get(&key) {
                  self.render_check_cell(ui, &result);
                }
              }
            });
          });
      });
  }

  fn render_check_cell(
    &self,
    ui: &mut Ui,
    result: &CheckResult,
  ) {
    let border_color = match result.status {
      Status::Healthy => COLOR_SUCCESS,
      Status::Unhealthy => COLOR_UNHEALTHY,
      Status::Error => COLOR_ERROR,
    };

    egui::Frame::default()
      .fill(COLOR_CARD_INNER)
      .inner_margin(egui::Margin::same(8.0))
      .rounding(egui::Rounding::same(4.0))
      .stroke(egui::Stroke::new(2.0, border_color))
      .show(ui, |ui| {
        ui.vertical(|ui| {
          // Name
          ui.label(
            RichText::new(&result.name)
              .color(COLOR_TEXT)
              .strong()
              .size(13.0),
          );

          // Version
          ui.label(
            RichText::new(
              result.version.as_deref().unwrap_or("-"),
            )
            .color(COLOR_TEXT_DIM)
            .size(11.0),
          );

          ui.add_space(4.0);

          // Status always shown
          let (icon, text, color) =
            Self::status_icon_and_text(&result.status);
          ui.horizontal(|ui| {
            ui.label(
              RichText::new(icon)
                .color(color)
                .size(13.0)
                .strong(),
            );
            ui.label(
              RichText::new(text).color(color).size(13.0),
            );
          });

          // Sub-checks only when not healthy
          if result.status != Status::Healthy
            && !result.sub_checks.is_empty()
          {
            ui.add_space(4.0);
            ui.separator();
            for sub in &result.sub_checks {
              let (sub_icon, sub_color) =
                if sub.status == "Healthy" {
                  ("✓", COLOR_SUCCESS)
                } else {
                  ("✗", COLOR_UNHEALTHY)
                };
              ui.horizontal(|ui| {
                ui.label(
                  RichText::new(sub_icon)
                    .color(sub_color)
                    .size(10.0),
                );
                ui.label(
                  RichText::new(&sub.name)
                    .color(COLOR_TEXT_DIM)
                    .size(10.0),
                );
              });
            }
          }
        });
      });
  }
}

impl eframe::App for DashboardApp {
  fn update(
    &mut self,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
  ) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = COLOR_BG;
    visuals.window_fill = COLOR_BG;
    ctx.set_visuals(visuals);

    egui::CentralPanel::default().show(ctx, |ui| {
      let available_width = ui.available_width();
      // Project header
      egui::Frame::default()
        .fill(COLOR_PRIMARY_DARK)
        .inner_margin(egui::Margin::symmetric(16.0, 10.0))
        .show(ui, |ui| {
          ui.set_min_width(available_width);
          ui.horizontal(|ui| {
            ui.label(
              RichText::new(&self.config.project_name)
                .color(COLOR_TEXT)
                .size(20.0)
                .strong(),
            );

            let freshness_text = self
              .cache
              .iter()
              .map(|entry| {
                entry.last_checked.elapsed().as_secs()
              })
              .min()
              .map(|secs| format!("{}s ago", secs))
              .unwrap_or_else(|| "no data yet".to_string());

            ui.with_layout(
              egui::Layout::right_to_left(
                egui::Align::Center,
              ),
              |ui| {
                ui.label(
                  RichText::new(freshness_text)
                    .color(COLOR_TEXT_DIM)
                    .size(11.0),
                );
              },
            );
          });
        });

      ui.add_space(12.0);

      for env in &self.config.environments {
        self.render_environment(ui, env, available_width);
        ui.add_space(8.0);
      }
    });

    ctx.request_repaint_after(Duration::from_secs(
      self.config.stale_timeout_seconds,
    ));
  }
}
