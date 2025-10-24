#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::team_creator::{Player, best_balanced_split, sum_skill};
use std::cmp::PartialEq;

mod team_creator;

use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily, RichText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 380.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Team Creator by Kallel",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "my_font".to_owned(),
                std::sync::Arc::new(
                    FontData::from_static(include_bytes!(
                        "../Stratum2WebMedium.otf"
                    )),
                ),
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "my_font".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
enum Tab {
    TeamCreator,
    Results,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AppData {
    players: Vec<Player>,
    zoom: f32,
}

struct MyApp {
    tab: Tab,
    zoom: f32,
    players: Vec<Player>,
    team1: Vec<Player>,
    team2: Vec<Player>,
    skill_difference: i32,
    error: bool,
    captain1_idx: Option<usize>,
    captain2_idx: Option<usize>,
}

impl Default for MyApp {
    fn default() -> Self {
        let data = MyApp::read_from_disk();
        MyApp {
            tab: Tab::TeamCreator,
            zoom: data.as_ref().map_or(1.2, |d| d.zoom),
            players: data.map_or(Vec::new(), |d| d.players),
            team1: Vec::new(),
            team2: Vec::new(),
            skill_difference: 0,
            error: false,
            captain1_idx: None,
            captain2_idx: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(4))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.visuals_mut().button_frame = false;
                    ui.heading("Team Creator");
                    ui.separator();
                    egui::widgets::global_theme_preference_switch(ui);
                    ui.separator();
                    ui.label("Zoom: ");

                    if !ui
                        .add(egui::Slider::new(&mut self.zoom, 0.8..=3.0))
                        .dragged()
                    {
                        ctx.set_pixels_per_point(self.zoom);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.tab {
            Tab::TeamCreator => {
                self.show_team_creator(ui);
            }
            Tab::Results => {
                self.show_results(ui);
            }
        });
    }
}

impl MyApp {
    fn show_team_creator(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Add player").clicked() {
                self.players.push(Player::default());
            }

            ui.label(format!("Players: {}", self.players.len()));
        });

        ui.separator();

        // Seleção de capitães
        ui.horizontal(|ui| {
            ui.label("Capitão Time 1:");
            egui::ComboBox::from_label("")
                .selected_text(
                    self.captain1_idx
                        .and_then(|idx| self.players.get(idx).map(|p| p.name.as_str()))
                        .unwrap_or("Nenhum")
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.captain1_idx, None, "Nenhum");
                    for (idx, player) in self.players.iter().enumerate() {
                        if !player.name.is_empty() {
                            ui.selectable_value(
                                &mut self.captain1_idx,
                                Some(idx),
                                &player.name,
                            );
                        }
                    }
                });

            ui.label("Capitão Time 2:");
            egui::ComboBox::from_label("")
                .selected_text(
                    self.captain2_idx
                        .and_then(|idx| self.players.get(idx).map(|p| p.name.as_str()))
                        .unwrap_or("Nenhum")
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.captain2_idx, None, "Nenhum");
                    for (idx, player) in self.players.iter().enumerate() {
                        if !player.name.is_empty() {
                            ui.selectable_value(
                                &mut self.captain2_idx,
                                Some(idx),
                                &player.name,
                            );
                        }
                    }
                });
        });

        ui.separator();

        for player in &mut self.players {
            ui.horizontal(|ui| {
                ui.label("Nome:");
                ui.add(egui::TextEdit::singleline(&mut player.name).desired_width(80.0));
                ui.label("Skill Level:");
                ui.add(egui::DragValue::new(&mut player.skill).range(0..=35000));
            });
        }

        ui.horizontal(|ui| {
            if ui.button("Create Teams").clicked() {
                if self.players.len() % 2 != 0 {
                    self.error = true;
                } else if self.captain1_idx.is_some() && self.captain2_idx.is_none() {
                    self.error = true;
                } else if self.captain1_idx.is_none() && self.captain2_idx.is_some() {
                    self.error = true;
                } else if self.captain1_idx == self.captain2_idx && self.captain1_idx.is_some() {
                    self.error = true;
                } else {
                    self.save_to_disk();
                    let (team1, team2, skill_diff) = best_balanced_split(
                        &mut self.players,
                        self.captain1_idx,
                        self.captain2_idx,
                    );
                    self.team1 = team1;
                    self.team2 = team2;
                    self.skill_difference = skill_diff;
                    self.error = false;
                    self.tab = Tab::Results;
                }
            }

            if self.error {
                let error_msg = if self.players.len() % 2 != 0 {
                    "Number of players must be even!"
                } else if self.captain1_idx.is_some() != self.captain2_idx.is_some() {
                    "Defina ambos os capitães ou nenhum!"
                } else {
                    "Os capitães devem ser jogadores diferentes!"
                };
                ui.label(
                    RichText::new(error_msg).color(egui::Color32::RED),
                );
            }
        });
    }

    fn show_results(&mut self, ui: &mut egui::Ui) {
        ui.heading("Teams Created:");
        ui.label(format!("Skill Difference: {}", self.skill_difference.abs()));

        ui.separator();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Team 1").underline().strong());
                    ui.label(format!(" ({})", sum_skill(self.team1.as_ref())));
                });
                for player in &self.team1 {
                    let name_display = if self.captain1_idx.is_some() &&
                        self.players.get(self.captain1_idx.unwrap()).map(|p| &p.name) == Some(&player.name) {
                        format!("⭐ {} ({})", player.name, player.skill)
                    } else {
                        format!("{} ({})", player.name, player.skill)
                    };
                    ui.label(name_display);
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Team 2").underline().strong());
                    ui.label(format!(" ({})", sum_skill(self.team2.as_ref())));
                });
                for player in &self.team2 {
                    let name_display = if self.captain2_idx.is_some() &&
                        self.players.get(self.captain2_idx.unwrap()).map(|p| &p.name) == Some(&player.name) {
                        format!("⭐ {} ({})", player.name, player.skill)
                    } else {
                        format!("{} ({})", player.name, player.skill)
                    };
                    ui.label(name_display);
                }
            });
        });

        if ui.button("Voltar").clicked() {
            self.tab = Tab::TeamCreator;
        }
    }

    fn save_to_disk(&self) {
        let data = AppData {
            players: self.players.clone(),
            zoom: self.zoom,
        };

        let ron_string = ron::to_string(&data).expect("Failed to serialize data to RON");
        std::fs::write("team_creator_cache.ron", ron_string)
            .expect("Failed to write cache to disk");
    }

    fn read_from_disk() -> Option<AppData> {
        let ron_string = std::fs::read_to_string("team_creator_cache.ron").ok()?;
        let data: AppData =
            ron::from_str(&ron_string).expect("Failed to deserialize data from RON");
        Some(data)
    }
}
