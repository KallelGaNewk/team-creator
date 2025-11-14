use crate::cache::PersistentCache;
use crate::views::Views;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::{CreationContext, egui};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq)]
pub enum ThemePreference {
    Light,
    Dark,
    System,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SettingsData {
    pub zoom: f32,
    pub is_updating_zoom: bool,
    pub theme: ThemePreference,
}

impl PersistentCache for SettingsData {
    fn filename() -> &'static str {
        "settings_cache.ron"
    }
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            zoom: 1.35,
            is_updating_zoom: false,
            theme: ThemePreference::System,
        }
    }
}

pub struct App {
    views: Views,
    settings: SettingsData,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.settings.is_updating_zoom {
            ctx.set_pixels_per_point(self.settings.zoom);
        }

        match self.settings.theme {
            ThemePreference::Light => ctx.set_theme(egui::Theme::Light),
            ThemePreference::Dark => ctx.set_theme(egui::Theme::Dark),
            ThemePreference::System => {
                ctx.set_theme(ctx.system_theme().unwrap_or(egui::Theme::Dark))
            }
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // ui.visuals_mut().button_frame = false;
                ui.heading("Kallel's Utilities");
                ui.separator();

                self.view_tabs(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    self.views.ui(ui, &mut self.settings);
                });
        });
    }
}

impl App {
    pub fn default(cc: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "my_font".to_owned(),
            std::sync::Arc::new(FontData::from_static(include_bytes!(
                "../assets/Stratum2WebMedium.otf"
            ))),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Self {
            views: Views::default(),
            settings: SettingsData::read_or(SettingsData::default()),
        }
    }

    fn view_tabs(&mut self, ui: &mut egui::Ui) {
        let mut selected_view = None;
        for (index, view) in self.views.views.iter().enumerate() {
            if ui
                .selectable_label(index == self.views.active_view, view.name())
                .clicked()
            {
                selected_view = Some(index);
            }
        }

        if let Some(index) = selected_view {
            self.views.set_active_view(index);
        }
    }
}
