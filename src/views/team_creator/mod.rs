mod team_creator;

use eframe::egui;
use eframe::egui::{CursorIcon, RichText};
use team_creator::{best_balanced_split, sum_skill, Player};

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
enum Tab {
    TeamCreator,
    Results,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {
    players: Vec<Player>,
    number_of_teams: usize,
}

pub struct TeamCreator {
    tab: Tab,
    teams: Vec<Vec<Player>>,
    persistent_data: PersistentData,
    player_being_edited: Option<usize>,
    new_player: Player,
    #[cfg(target_arch = "wasm32")]
    window: web_sys::Window,
}

impl super::View for TeamCreator {
    fn name(&self) -> &str {
        "👥 Team Creator"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match self.tab {
            Tab::TeamCreator => {
                self.show_team_creator(ui);
            }
            Tab::Results => {
                self.show_results(ui);
            }
        }
    }
}

impl Default for PersistentData {
    fn default() -> Self {
        PersistentData {
            players: Vec::new(),
            number_of_teams: 2,
        }
    }
}

impl Default for TeamCreator {
    fn default() -> Self {
        TeamCreator {
            tab: Tab::TeamCreator,
            teams: Vec::new(),
            persistent_data: TeamCreator::read_from_disk().unwrap_or_default(),
            player_being_edited: None,
            new_player: Player::default(),
            #[cfg(target_arch = "wasm32")]
            window: web_sys::window().expect("no global `window` exists"),
        }
    }
}

impl TeamCreator {
    fn show_team_creator(&mut self, ui: &mut egui::Ui) {
        let hide_skills = self.hide_skills();

        ui.horizontal(|ui| {
            ui.label(format!("Players: {}", self.persistent_data.players.len()));

            ui.separator();

            ui.label("Teams: ");
            if ui
                .add(
                    egui::DragValue::new(&mut self.persistent_data.number_of_teams)
                        .speed(0.1)
                        .range(2..=20),
                )
                .changed()
            {
                for player in &mut self.persistent_data.players {
                    player.is_captain = false;
                }
            }

            let players_per_team: f32 = if self.persistent_data.number_of_teams == 0 {
                0.0
            } else {
                self.persistent_data.players.len() as f32
                    / self.persistent_data.number_of_teams as f32
            };

            if players_per_team.fract() != 0.0 {
                ui.colored_label(
                    egui::Color32::RED,
                    format!("({:.2} players per team)", players_per_team),
                );
            } else {
                ui.label(format!("({:.0} players per team)", players_per_team));
            }

            ui.separator();

            self.reset_button(ui);
        });

        ui.separator();
        let number_of_captains = self
            .persistent_data
            .players
            .iter()
            .filter(|p| p.is_captain)
            .count()
            + usize::from(self.new_player.is_captain);

        ui.add_enabled_ui(self.player_being_edited == None, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                let text_box = ui
                    .add(egui::TextEdit::singleline(&mut self.new_player.name).desired_width(80.0));
                ui.label("Skill Level:");
                ui.add(egui::DragValue::new(&mut self.new_player.skill).range(0..=35000));

                if number_of_captains < self.persistent_data.number_of_teams
                    || self.new_player.is_captain
                {
                    ui.label("Captain:");
                    ui.checkbox(&mut self.new_player.is_captain, "");
                }

                if ui.button("➕ Add").clicked()
                    || (text_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    self.persistent_data.players.push(self.new_player.clone());
                    self.new_player = Player::default();
                    text_box.request_focus();
                }
            });
        });

        ui.separator();

        let mut to_remove = Vec::new();
        for (idx, player) in self.persistent_data.players.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                if self.player_being_edited == Some(idx) {
                    ui.label("Name:");
                    let text_box =
                        ui.add(egui::TextEdit::singleline(&mut player.name).desired_width(80.0));
                    ui.label("Skill Level:");
                    ui.add(egui::DragValue::new(&mut player.skill).range(0..=35000));

                    if number_of_captains < self.persistent_data.number_of_teams
                        || player.is_captain
                    {
                        ui.label("Captain:");
                        ui.checkbox(&mut player.is_captain, "");
                    }

                    if ui.button("💾").clicked()
                        || (text_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.player_being_edited = None;
                    }
                } else {
                    if ui.button("🗑").clicked() {
                        to_remove.push(idx);
                        return;
                    }

                    let prev_button_frame = ui.visuals().button_frame;
                    ui.visuals_mut().button_frame = false;
                    if ui
                        .button(player.pretty_name(hide_skills))
                        .on_hover_cursor(CursorIcon::Text)
                        .clicked()
                    {
                        self.player_being_edited = Some(idx);
                    };
                    ui.visuals_mut().button_frame = prev_button_frame;
                }
            });
        }

        for idx in to_remove.into_iter().rev() {
            self.persistent_data.players.remove(idx);
        }

        ui.horizontal(|ui| {
            if number_of_captains != 0
                && (number_of_captains % self.persistent_data.number_of_teams != 0
                    || number_of_captains < self.persistent_data.number_of_teams)
                || self.persistent_data.players.len() % self.persistent_data.number_of_teams != 0
                || self.persistent_data.players.len() == 0
            {
                ui.add_enabled(false, egui::Button::new("Create Teams"));
            } else {
                if ui.button("Create Teams").clicked() {
                    self.save_to_disk();
                    self.teams = best_balanced_split(
                        &mut self.persistent_data.players,
                        self.persistent_data.number_of_teams,
                    );
                    self.tab = Tab::Results;
                }
            }
        });
    }

    fn show_results(&mut self, ui: &mut egui::Ui) {
        let hide_skills = self.hide_skills();

        ui.horizontal(|ui| {
            ui.heading("Teams Created:");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.reset_button(ui);
            });
        });

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

                        if !hide_skills {
                            ui.label(format!("{}", sum_skill(team)));
                        }
                    });

                    for (player_idx, player) in team.iter().enumerate() {
                        let prev_button_frame = ui.visuals().button_frame;
                        ui.visuals_mut().button_frame = false;

                        ui.menu_button(player.pretty_name(hide_skills), |ui| {
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

                                    if ui
                                        .selectable_label(
                                            false,
                                            other_player.pretty_name(hide_skills),
                                        )
                                        .clicked()
                                    {
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
                self.teams = best_balanced_split(
                    &mut self.persistent_data.players,
                    self.persistent_data.number_of_teams,
                );
            }
            if ui.button("📋 Copy").clicked() {
                self.copy_teams_to_clipboard(ui);
            }
        });
    }

    fn hide_skills(&mut self) -> bool {
        self.persistent_data.players.iter().all(|p| p.skill == 0)
    }

    fn reset_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("↻ Reset").clicked() {
            self.persistent_data.players.clear();
            self.teams.clear();
            self.persistent_data.number_of_teams = 2;
            self.tab = Tab::TeamCreator;
            self.save_to_disk();
        }
    }

    fn save_to_disk(&self) {
        let ron_string =
            ron::to_string(&self.persistent_data).expect("Failed to serialize data to RON");

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

    fn read_from_disk() -> Option<PersistentData> {
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

        let (team_a, team_b) = if t1 < t2 {
            let (left, right) = self.teams.split_at_mut(t2);
            (&mut left[t1], &mut right[0])
        } else {
            let (left, right) = self.teams.split_at_mut(t1);
            (&mut right[0], &mut left[t2])
        };

        if p1 < team_a.len() && p2 < team_b.len() {
            std::mem::swap(&mut team_a[p1], &mut team_b[p2]);
        }
    }

    fn copy_teams_to_clipboard(&mut self, ui: &mut egui::Ui) {
        let hide = self.hide_skills();
        let mut output = String::new();

        for (team_idx, team) in self.teams.iter().enumerate() {
            let team_header = if hide {
                format!("Team {}:\n", team_idx + 1)
            } else {
                format!("Team {} ({}):\n", team_idx + 1, sum_skill(team))
            };
            output.push_str(&team_header);

            for player in team {
                output.push_str(&format!("- {}\n", player.pretty_name(hide)));
            }

            output.push('\n');
        }

        ui.ctx().copy_text(output);
    }
}
