use crate::str_ext::StringExt;
use crate::team_creator::{Player, best_balanced_split, sum_skill};
use eframe::egui::{FontData, FontDefinitions, FontFamily, RichText};
use eframe::{CreationContext, egui};

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

pub struct MyApp {
    tab: Tab,
    zoom: f32,
    players: Vec<Player>,
    teams: Vec<Vec<Player>>,
    number_of_teams: usize,
    #[cfg(target_arch = "wasm32")]
    window: web_sys::Window,
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
    pub fn create(cc: &CreationContext) -> Self {
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

        let data = MyApp::read_from_disk();
        MyApp {
            tab: Tab::TeamCreator,
            zoom: data.as_ref().map_or(1.2, |d| d.zoom),
            players: data.as_ref().map_or(Vec::new(), |d| d.players.clone()),
            teams: Vec::new(),
            number_of_teams: data.map_or(2, |d| d.number_of_teams),
            #[cfg(target_arch = "wasm32")]
            window: web_sys::window().expect("no global `window` exists"),
        }
    }

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

                ui.label("Name:");
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
                        ui.label(format!("{}", sum_skill(team)));
                    });

                    for (player_idx, player) in team.iter().enumerate() {
                        let prev_button_frame = ui.visuals().button_frame;
                        ui.visuals_mut().button_frame = false;

                        let button_label = format!(
                            "{}{} ({})",
                            if player.is_captain { "⭐ " } else { "" },
                            player.name.as_str_or("<unnamed>"),
                            player.skill
                        );

                        ui.menu_button(button_label, |ui| {
                            for (other_team_idx, other_team) in teams_snapshot.iter().enumerate() {
                                if other_team_idx == team_idx {
                                    continue;
                                }

                                ui.label(format!("Team {}", other_team_idx + 1));

                                for (other_player_idx, other_player) in
                                    other_team.iter().enumerate()
                                {
                                    if other_player.is_captain {
                                        continue;
                                    }

                                    let label = format!(
                                        "{}{} ({})",
                                        if other_player.is_captain { "⭐ " } else { "" },
                                        other_player.name.as_str_or("<unnamed>"),
                                        other_player.skill
                                    );

                                    if ui.selectable_label(false, label).clicked() {
                                        self.swap_players(
                                            team_idx,
                                            player_idx,
                                            other_team_idx,
                                            other_player_idx,
                                        );
                                        ui.close();
                                        return;
                                    }
                                }

                                ui.separator();
                            }
                        });

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
            if ui.button("📋 Copy").clicked() {
                self.copy_teams_to_clipboard(ui);
            }
        });
    }

    fn save_to_disk(&self) {
        let ron_string = ron::to_string(&AppData {
            players: self.players.clone(),
            number_of_teams: self.number_of_teams,
            zoom: self.zoom,
        })
        .expect("Failed to serialize data to RON");

        #[cfg(not(target_arch = "wasm32"))]
        std::fs::write("team_creator_cache.ron", ron_string)
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
            let appdata = std::fs::read("team_creator_cache.ron").ok()?;
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

    /// Swap two players between different teams safely. If the teams are the same, do nothing.
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

    fn copy_teams_to_clipboard(&self, ui: &mut egui::Ui) {
        let mut output = String::new();

        for (team_idx, team) in self.teams.iter().enumerate() {
            output.push_str(&format!("Team {} ({}):\n", team_idx + 1, sum_skill(team)));
            for player in team {
                output.push_str(&format!(
                    "- {}{} ({})\n",
                    if player.is_captain { "⭐ " } else { "" },
                    player.name.as_str_or("<unnamed>"),
                    player.skill
                ));
            }
            output.push('\n');
        }

        ui.ctx().copy_text(output);
    }
}
