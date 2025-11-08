use eframe::egui;

#[derive(serde::Serialize, serde::Deserialize)]
struct AppData {}

pub struct Settings {}

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
        // let data = SpinWheel::read_from_disk();
        Settings {}
    }
}

impl Settings {
    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("↻ Reset").clicked() {
            self.save_to_disk();
        }
    }

    fn save_to_disk(&self) {
        let ron_string = ron::to_string(&AppData {}).expect("Failed to serialize data to RON");

        #[cfg(not(target_arch = "wasm32"))]
        std::fs::write("../../../team_creator_cache.ron", ron_string)
            .expect("Failed to write cache to disk");

        #[cfg(target_arch = "wasm32")]
        if let Some(storage) = self.window.local_storage().ok().flatten() {
            storage
                .set_item("team_creator_cache", &ron_string)
                .expect("Failed to write cache to localStorage");
        }
    }

    fn read_from_disk() -> Option<AppData> {
        #[cfg(not(target_arch = "wasm32"))]
        let ron_str = {
            let appdata = std::fs::read("../../../team_creator_cache.ron").ok()?;
            String::from_utf8(appdata).ok()?
        };

        #[cfg(target_arch = "wasm32")]
        let ron_str = {
            if let Some(storage) = web_sys::window()
                .expect("no global `window` exists")
                .local_storage()
                .ok()
                .flatten()
            {
                storage.get_item("team_creator_cache").ok().flatten()?
            } else {
                return None;
            }
        };

        ron::from_str(&ron_str).ok()
    }
}
