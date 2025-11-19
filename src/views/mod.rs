use crate::app::SettingsData;

mod lambidinha;
mod settings;
mod spin_wheel;
mod team_creator;

pub trait View {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut eframe::egui::Ui, settings: &mut SettingsData);
}

pub struct Views {
    pub active_view: usize,
    pub views: Vec<Box<dyn View>>,
}

impl Default for Views {
    fn default() -> Self {
        Self {
            active_view: 0,
            views: vec![
                Box::<team_creator::TeamCreator>::default(),
                Box::<spin_wheel::SpinWheel>::default(),
                Box::<settings::Settings>::default(),
                Box::<lambidinha::Lambidinha>::default(),
            ],
        }
    }
}

impl Views {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.views[self.active_view].name()
    }

    pub fn ui(&mut self, ui: &mut eframe::egui::Ui, settings: &mut SettingsData) {
        self.views[self.active_view].ui(ui, settings);
    }

    pub fn set_active_view(&mut self, index: usize) {
        if index < self.views.len() {
            self.active_view = index;
        }
    }
}
