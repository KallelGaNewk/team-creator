use crate::cache::PersistentCache;
use eframe::egui;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "template_cache.ron"
    }
}

pub struct Settings {
    persistent_data: PersistentData,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            persistent_data: PersistentData::read_or(PersistentData {}),
        }
    }
}

impl super::View for Settings {
    fn name(&self) -> &str {
        "📄 Template"
    }

    fn ui(&mut self, ui: &mut egui::Ui, _settings: &mut crate::app::SettingsData) {
        ui.heading("Template View");
    }
}

impl Settings {}
