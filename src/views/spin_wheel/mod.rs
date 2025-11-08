mod constants;
mod wheel;

use crate::cache::PersistentCache;
use crate::extensions::PressedEnterExt;
use crate::views::spin_wheel::wheel::{Choice, Wheel};
use eframe::egui;
use eframe::egui::{Color32, FontId, Id, Modal, Pos2, Stroke};
use eframe::epaint::PathShape;

#[derive(serde::Serialize, serde::Deserialize)]
struct PersistentData {}

pub struct SpinWheel {
    persistent_data: PersistentData,
    wheel: Wheel,
    input_text: String,
    wheel_choices: Vec<Choice>,
    removed_choices: Vec<Choice>,
}

impl PersistentCache for PersistentData {
    fn filename() -> &'static str {
        "spin_wheel_cache.ron"
    }
}

impl Default for SpinWheel {
    fn default() -> Self {
        SpinWheel {
            persistent_data: PersistentData::read_or(PersistentData {}),
            wheel: Wheel::new(),
            input_text: String::new(),
            wheel_choices: vec![],
            removed_choices: vec![],
        }
    }
}

impl super::View for SpinWheel {
    fn name(&self) -> &str {
        "🎲 Spin Wheel"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(winner) = self.wheel.winner.clone() {
            let modal = Modal::new(Id::new("Result Modal")).show(ui.ctx(), |ui| {
                ui.set_width(250.0);
                ui.heading("Result:");
                ui.separator();
                ui.heading(egui::RichText::new(&winner.label).size(24.0));
                ui.separator();
                egui::Sides::new().show(ui, |_ui| {}, |ui| {
                    if ui.button("🗑 Remove").clicked() {
                        self.remove_entry(winner, true);
                        ui.close();
                    }

                    if ui.button("❌ Close").clicked() {
                        ui.close();
                    }
                });
            });

            if modal.should_close() {
                self.wheel.winner = None;
            }
        }

        let available_rect = ui.max_rect();
        let painter = ui.painter();

        // Tick the wheel
        self.wheel.do_spin(ui.ctx(), &mut self.wheel_choices);

        self.wheel.center = egui::pos2(
            available_rect.width() * 0.25 + constants::WHEEL_OFFSET,
            available_rect.center().y,
        );

        let available_width = available_rect.width() / 4.0;
        let available_height = available_rect.height() / 2.0;
        self.wheel.radius = f32::min(available_width, available_height);
        self.wheel.draw(painter, &mut self.wheel_choices);

        // Triangle
        if !self.wheel_choices.is_empty() {
            let triangle_center = self.wheel.get_triangle_center();
            let triangle_points: Vec<Pos2> = vec![
                egui::pos2(triangle_center.x - 15.0, triangle_center.y),
                egui::pos2(triangle_center.x + 30.0, triangle_center.y + 20.0),
                egui::pos2(triangle_center.x + 30.0, triangle_center.y - 20.0),
            ];
            let path = PathShape::convex_polygon(
                triangle_points,
                Color32::from_rgb(200, 200, 200),
                Stroke::NONE,
            );
            painter.add(path);
        }

        // Inputs
        let inputs_center = egui::pos2(
            available_rect.width() * 0.75 + constants::WHEEL_OFFSET,
            available_rect.center().y,
        );

        let inputs_width = available_rect.width() / 2.0 * 0.8 - constants::WHEEL_OFFSET;
        let inputs_height = available_rect.height() * 0.8;
        let inputs_pos = egui::pos2(
            inputs_center.x - inputs_width / 2.0,
            inputs_center.y - inputs_height / 2.0,
        );

        ui.add_space(constants::SPACER_AMOUNT);

        // Add input UI to the right side of the container
        egui::Area::new(egui::Id::new("inputs"))
            .fixed_pos(inputs_pos)
            .show(ui.ctx(), |ui| {
                ui.set_height(inputs_height);
                ui.set_width(inputs_width);

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        let button_width: f32 = ui.spacing().interact_size.x;
                        let input_width: f32 = ui.available_width() - button_width;
                        let text_box = ui.add_enabled(
                            self.can_type_entry(),
                            egui::TextEdit::singleline(&mut self.input_text)
                                .hint_text("Add a choice")
                                .char_limit(constants::MAX_INPUT_SIZE)
                                .desired_width(input_width)
                                .desired_rows(1),
                        );

                        if ui
                            .add_enabled(self.can_add_entry(), egui::Button::new("➕ Add"))
                            .clicked()
                            || text_box.pressed_enter(ui.ctx())
                        {
                            self.add_entry();
                            text_box.request_focus();
                        }
                    });

                    ui.add_space(constants::SPACER_AMOUNT);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Choices:");
                            ui.add_space(constants::SPACER_AMOUNT / 2.0);
                            let choices_to_display: Vec<Choice> = self.wheel_choices.clone();
                            for choice in choices_to_display {
                                ui.horizontal(|ui| {
                                    if ui.button("❌").clicked() {
                                        self.remove_entry(choice.clone(), false);
                                    }
                                    ui.label(&choice.label);
                                });
                            }
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.label("Removed:");
                            ui.add_space(constants::SPACER_AMOUNT / 2.0);
                            let removed_to_display: Vec<Choice> = self.removed_choices.clone();
                            for choice in removed_to_display {
                                ui.horizontal(|ui| {
                                    if ui.button("🔙 Add back").clicked() {
                                        self.add_entry_back(choice.clone());
                                    }
                                    ui.label(&choice.label);
                                });
                            }
                        });
                    });

                    if ui
                        .add_enabled(
                            !self.wheel.spinning && !self.wheel_choices.is_empty(),
                            egui::Button::new(
                                egui::RichText::new("💫 Spin the wheel!")
                                    .font(FontId::proportional(constants::TITLE_SIZE)),
                            ),
                        )
                        .clicked()
                    {
                        self.wheel.start_spin();
                    }

                    if ui.button("🗑 Clear").clicked() {
                        self.wheel_choices = vec![];
                        self.removed_choices = vec![];
                        self.wheel.clear();
                    }
                });
            });
    }
}

impl SpinWheel {
    /// Adds a new entry to the wheel choices
    fn add_entry(&mut self) {
        if self.can_add_entry() {
            let new_choice = Choice::new(self.input_text.trim().replace("\n", " "));
            self.wheel_choices.push(new_choice);
            self.wheel.reset_rotation(&self.wheel_choices);
            self.input_text.clear();
        }
    }

    /// Removes an entry and adds it to the removed choices list
    fn remove_entry(&mut self, choice: Choice, soft: bool) {
        let entry_index = self
            .wheel_choices
            .iter()
            .position(|entry_found| entry_found.label == choice.label);

        if let Some(index) = entry_index {
            self.wheel_choices.remove(index);
            if soft {
                self.removed_choices.push(choice);
            }
        }

        self.wheel.reset_rotation(&self.wheel_choices);
    }

    /// Adds an entry back from the removed choices list
    fn add_entry_back(&mut self, choice: Choice) {
        let entry_index = self
            .removed_choices
            .iter()
            .position(|entry_found| entry_found.label == choice.label);

        if let Some(index) = entry_index {
            self.removed_choices.remove(index);
            self.wheel_choices.push(choice);
        }

        self.wheel.reset_rotation(&self.wheel_choices);
    }

    /// Checks if a new entry can be added
    fn can_add_entry(&self) -> bool {
        self.can_type_entry() && !self.input_text.is_empty()
    }
    /// Checks if typing a new entry is allowed
    fn can_type_entry(&self) -> bool {
        !self.wheel.spinning && !self.choices_full()
    }
    /// Checks if the wheel choices have reached the maximum limit
    fn choices_full(&self) -> bool {
        self.wheel_choices.len() >= constants::MAX_CHOICES
    }
}
