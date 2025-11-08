use crate::cache::PersistentCache;
use eframe::egui;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "spin_wheel_cache.ron"
    }
}

pub struct SpinWheel {
    persistent_data: PersistentData,
}

impl super::View for SpinWheel {
    fn name(&self) -> &str {
        "🎲 Spin Wheel"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Spin Wheel");
        ui.label("This is the Spin Wheel view.");
        self.reset_button(ui);
    }
}

impl Default for SpinWheel {
    fn default() -> Self {
        SpinWheel {
            persistent_data: PersistentData::read_or(PersistentData {}),
        }
    }
}

impl SpinWheel {
    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("↻ Reset").clicked() {
            self.persistent_data.save_to_disk();
        }
    }
}
