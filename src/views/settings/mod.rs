use crate::cache::PersistentCache;
use eframe::egui;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "settings_cache.ron"
    }
}

pub struct Settings {
    persistent_data: PersistentData,
}

impl super::View for Settings {
    fn name(&self) -> &str {
        "⚙ Settings"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            persistent_data: PersistentData::read_or(PersistentData {}),
        }
    }
}

impl Settings {
    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("↻ Reset").clicked() {
            self.persistent_data.save_to_disk();
        }
    }
}
