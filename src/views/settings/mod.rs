#![allow(dead_code)]

use crate::cache::PersistentCache;
use eframe::egui;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "settings_cache.ron"
    }
}

pub struct Settings {}

impl super::View for Settings {
    fn name(&self) -> &str {
        "⚙ Settings"
    }

    fn ui(&mut self, ui: &mut egui::Ui, settings: &mut crate::app::SettingsData) {
        egui::Sides::new().show(
            ui,
            |ui| {
                ui.heading("Settings");
            },
            |ui| {
                self.reset_button(ui, settings);
            },
        );
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Zoom:");
            if !ui
                .add(egui::Slider::new(&mut settings.zoom, 0.8..=3.0))
                .dragged()
            {
                ui.ctx().set_pixels_per_point(settings.zoom);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Theme:");
            egui::widgets::global_theme_preference_buttons(ui);
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {}
    }
}

impl Settings {
    fn reset_button(&mut self, ui: &mut egui::Ui, settings: &mut crate::app::SettingsData) {
        if ui.button("↻ Reset").clicked() {
            *settings = crate::app::SettingsData::default();
        }
    }
}
