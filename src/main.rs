#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::team_creator::{Player, best_balanced_split, sum_skill};

mod team_creator;

use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily, RichText};
use image::GenericImageView;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let icon_data = {
        let image = image::load_from_memory(include_bytes!("../assets/icon.ico"))
            .expect("Failed to load icon image");
        let rgba = image.to_rgba8().into_vec();
        let (width, height) = image.dimensions();
        egui::IconData {
            rgba,
            width,
            height,
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([510.0, 430.0])
            .with_inner_size([510.0, 430.0])
            .with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "Team Creator by Kallel",
        options,
        Box::new(|cc| {
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

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
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

                    Ok(Box::<MyApp>::default())
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
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
    number_of_teams: usize,
}

struct MyApp {
    tab: Tab,
    zoom: f32,
    players: Vec<Player>,
    teams: Vec<Vec<Player>>,
    number_of_teams: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let data = MyApp::read_from_disk();
        MyApp {
            tab: Tab::TeamCreator,
            zoom: data.as_ref().map_or(1.2, |d| d.zoom),
            players: data.as_ref().map_or(Vec::new(), |d| d.players.clone()),
            teams: Vec::new(),
            number_of_teams: data.map_or(2, |d| d.number_of_teams),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
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

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("↻ Reset").clicked() {
                        self.players.clear();
                        self.teams.clear();
                        self.number_of_teams = 2;
                        self.tab = Tab::TeamCreator;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| match self.tab {
                    Tab::TeamCreator => {
                        self.show_team_creator(ui);
                    }
                    Tab::Results => {
                        self.show_results(ui);
                    }
                });
        });
    }
}

impl MyApp {
    fn show_team_creator(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("➕ Add player").clicked() {
                self.players.push(Player::default());
            }

            ui.label(format!("Players: {}", self.players.len()));

            ui.separator();

            ui.label("Teams: ");
            if ui
                .add(
                    egui::DragValue::new(&mut self.number_of_teams)
                        .speed(0.1)
                        .range(2..=20),
                )
                .changed()
            {
                for player in &mut self.players {
                    player.is_captain = false;
                }
            }

            let players_per_team: f32 = if self.number_of_teams == 0 {
                0.0
            } else {
                self.players.len() as f32 / self.number_of_teams as f32
            };

            if players_per_team.fract() != 0.0 {
                ui.colored_label(
                    egui::Color32::RED,
                    format!("({:.2} players per team)", players_per_team),
                );
            } else {
                ui.label(format!("({:.0} players per team)", players_per_team));
            }
        });

        ui.separator();

        let number_of_captains = self.players.iter().filter(|p| p.is_captain).count();
        let mut to_remove = Vec::new();
        for (idx, player) in self.players.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if ui.button("❌").clicked() {
                    to_remove.push(idx);
                    return;
                }

                ui.label("Nome:");
                ui.add(egui::TextEdit::singleline(&mut player.name).desired_width(80.0));
                ui.label("Skill Level:");
                ui.add(egui::DragValue::new(&mut player.skill).range(0..=35000));

                if number_of_captains < self.number_of_teams || player.is_captain {
                    ui.label("Captain:");
                    ui.checkbox(&mut player.is_captain, "");
                }
            });
        }

        for idx in to_remove.into_iter().rev() {
            self.players.remove(idx);
        }

        ui.horizontal(|ui| {
            if number_of_captains != 0
                && (number_of_captains % self.number_of_teams != 0
                    || number_of_captains < self.number_of_teams)
                || self.players.len() % self.number_of_teams != 0
                || self.players.len() == 0
            {
                ui.add_enabled(false, egui::Button::new("Create Teams"));
            } else {
                if ui.button("Create Teams").clicked() {
                    self.save_to_disk();
                    self.teams = best_balanced_split(&mut self.players, self.number_of_teams);
                    self.tab = Tab::Results;
                }
            }
        });
    }

    fn show_results(&mut self, ui: &mut egui::Ui) {
        ui.heading("Teams Created:");

        // Make a snapshot clone for read-only iteration so we can mutate self.teams when swapping
        let teams_snapshot = self.teams.clone();

        ui.horizontal(|ui| {
            for (team_idx, team) in teams_snapshot.iter().enumerate() {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            RichText::new(format!("Team {}", team_idx + 1))
                                .underline()
                                .strong(),
                        );

                        ui.label(format!("{}", sum_skill(team.as_ref())));
                    });

                    for (player_idx, player) in team.iter().enumerate() {
                        // For each player we show a menu button (dropdown) that lists eligible players
                        // from other teams. We temporarily disable button framing to match the style.
                        let prev_button_frame = ui.visuals().button_frame;
                        ui.visuals_mut().button_frame = false;

                        let button_label = format!(
                            "{}{} ({})",
                            if player.is_captain { "⭐ " } else { "" },
                            if player.name.is_empty() { "<unnamed>" } else { &player.name },
                            player.skill
                        );

                        // Use menu_button to create a dropdown. The id_source ensures uniqueness.
                        ui.menu_button(button_label, |ui| {
                                 // List all players from other teams as selectable entries
                                 let mut done = false;
                                 'other_teams: for (other_team_idx, other_team) in
                                    teams_snapshot.iter().enumerate()
                                 {
                                     if other_team_idx == team_idx {
                                        continue; // skip same team
                                     }

                                     ui.label(format!("Team {}", other_team_idx + 1));

                                     for (other_player_idx, other_player) in
                                        other_team.iter().enumerate()
                                     {
                                        // Skip captain players
                                        if other_player.is_captain {
                                            continue;
                                        }

                                         let label = format!(
                                             "{}{} ({})",
                                             if other_player.is_captain { "⭐ " } else { "" },
                                             if other_player.name.is_empty() { "<unnamed>" } else { &other_player.name },
                                             other_player.skill
                                         );

                                         if ui.selectable_label(false, label).clicked() {
                                             // Perform swap between (team_idx, player_idx) and
                                             // (other_team_idx, other_player_idx)
                                             self.swap_players(
                                                 team_idx,
                                                 player_idx,
                                                 other_team_idx,
                                                 other_player_idx,
                                             );

                                             // Close the menu and break out of loops
                                             ui.close();
                                             done = true;
                                             break;
                                         }
                                     }

                                     if done {
                                         break 'other_teams;
                                     } else {
                                         ui.separator();
                                     }
                                 }
                             });

                        // Restore previous visuals.button_frame value
                        ui.visuals_mut().button_frame = prev_button_frame;
                    }
                });

                if team_idx < self.teams.len() - 1 {
                    ui.separator();
                }
            }
        });

        ui.horizontal(|ui| {
            if ui.button("⬅ Back").clicked() {
                self.tab = Tab::TeamCreator;
            }
            if ui.button("🔄 Recreate").clicked() {
                self.teams = best_balanced_split(&mut self.players, self.number_of_teams);
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_to_disk(&self) {
        let ron_string = ron::to_string(&AppData {
            players: self.players.clone(),
            number_of_teams: self.number_of_teams,
            zoom: self.zoom,
        })
        .expect("Failed to serialize data to RON");

        std::fs::write("team_creator_cache.ron", ron_string)
            .expect("Failed to write cache to disk");
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_from_disk() -> Option<AppData> {
        let appdata = std::fs::read("team_creator_cache.ron").ok()?;
        let ron_str = String::from_utf8(appdata).ok()?;
        ron::from_str(&ron_str).ok()
    }

    #[cfg(target_arch = "wasm32")]
    fn save_to_disk(&self) {
        // No-op on wasm
    }

    #[cfg(target_arch = "wasm32")]
    fn read_from_disk() -> Option<AppData> {
        None
    }

    // Swap two players between different teams safely. If the teams are the same, do nothing.
    fn swap_players(&mut self, t1: usize, p1: usize, t2: usize, p2: usize) {
        if t1 == t2 {
            return;
        }

        // Use split_at_mut to get two non-overlapping mutable slices so we can borrow both teams
        if t1 < t2 {
            let (left, right) = self.teams.split_at_mut(t2);
            // left[t1] and right[0] correspond to team t1 and team t2 respectively
            if p1 < left[t1].len() && p2 < right[0].len() {
                std::mem::swap(&mut left[t1][p1], &mut right[0][p2]);
            }
        } else {
            let (left, right) = self.teams.split_at_mut(t1);
            // right[0] is team t1, left[t2] is team t2
            if p1 < right[0].len() && p2 < left[t2].len() {
                std::mem::swap(&mut right[0][p1], &mut left[t2][p2]);
            }
        }
    }
}
