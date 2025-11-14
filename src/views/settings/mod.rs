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
            let slider = ui.add(egui::Slider::new(&mut settings.zoom, 0.8..=5.0));

            settings.is_updating_zoom = slider.dragged();
            ui.label(slider.drag_stopped().to_string());
            if slider.drag_stopped() {
                settings.save_to_disk();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Theme:");
            self.theme_picker(ui, settings);
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {}
    }
}

impl Settings {
    fn theme_picker(&mut self, ui: &mut egui::Ui, settings: &mut crate::app::SettingsData) {
        if ui
            .horizontal(|ui| {
                ui.selectable_value(
                    &mut settings.theme,
                    crate::app::ThemePreference::System,
                    "💻 System",
                )
                .clicked()
                    || ui
                        .selectable_value(
                            &mut settings.theme,
                            crate::app::ThemePreference::Dark,
                            "🌙 Dark",
                        )
                        .clicked()
                    || ui
                        .selectable_value(
                            &mut settings.theme,
                            crate::app::ThemePreference::Light,
                            "☀ Light",
                        )
                        .clicked()
            })
            .inner
        {
            settings.save_to_disk();
        }
    }

    fn reset_button(&mut self, ui: &mut egui::Ui, settings: &mut crate::app::SettingsData) {
        if ui.button("↻ Reset").clicked() {
            *settings = crate::app::SettingsData::default();
            settings.save_to_disk();
        }
    }
}
