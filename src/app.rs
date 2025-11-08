use crate::views::Views;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::{CreationContext, egui};

pub struct App {
    views: Views,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.heading("Kallel's Utilities");
                ui.separator();

                self.view_tabs(ui);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.separator();
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    self.views.ui(ui);
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
