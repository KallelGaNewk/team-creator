use crate::views::spin_wheel::constants;
use eframe::{
    egui::{self, Color32, Context, FontId, Painter, Pos2, Stroke},
    epaint::PathShape,
};
use egui::{Align2, epaint::TextShape};
use rand::Rng;
use std::f32::consts::PI;
use ulid::Ulid;

pub struct Wheel {
    pub radius: f32,
    pub center: Pos2,
    pub spinning: bool,
    pub winner: Option<Choice>,
    rotation: f32,
    spin_velocity: f32,
    pub selected_winner_once: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Choice {
    pub id: Ulid,
    pub label: String,
    pub weight: u32,
}

impl Choice {
    pub fn new(label: String, weight: Option<u32>) -> Self {
        Self {
            label: label.to_string(),
            weight: weight.unwrap_or(1),
            id: Ulid::new(),
        }
    }
}

impl Wheel {
    pub fn new() -> Self {
        Self {
            center: Pos2::default(),
            radius: 0.0,
            rotation: 0.0,
            winner: None,
            selected_winner_once: true,
            spinning: false,
            spin_velocity: 0.0,
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn do_spin(&mut self, ctx: &Context, wheel_choices: &mut Vec<Choice>) {
        self.rotation += self.spin_velocity;
        self.spin_velocity *= constants::BREAKING_PERCENT;

        if self.spin_velocity.abs() < constants::MIN_SPEED {
            self.spin_velocity = 0.0;
            self.spinning = false;

            if let Some(choice) = self.get_winner(wheel_choices) {
                if !self.selected_winner_once {
                    self.winner = Some(choice);
                    self.selected_winner_once = true;
                }
            }
        }

        ctx.request_repaint();
    }

    pub fn start_spin(&mut self) {
        if !self.spinning {
            self.selected_winner_once = false;
            self.rotation = rand::rng().random_range(0.0..(2.0 * PI));
            self.spin_velocity = rand::rng()
                .random_range(constants::SPIN_VELOCITY_MIN..constants::SPIN_VELOCITY_MAX);
            self.spinning = true;
        }
    }

    pub fn draw(&mut self, painter: &Painter, wheel_choices: &mut Vec<Choice>) {
        let colors = [
            Color32::from_rgb(51, 105, 232),
            Color32::from_rgb(213, 15, 37),
            Color32::from_rgb(238, 178, 17),
            Color32::from_rgb(0, 153, 37),
        ];

        if wheel_choices.is_empty() {
            painter.text(
                self.center,
                Align2::CENTER_CENTER,
                "Add options to spin!",
                FontId::proportional(30.0),
                Color32::WHITE,
            );
            return;
        }

        let total_weight = Wheel::get_total_weight(wheel_choices);
        let angle_step = 2.0 * PI / total_weight as f32;
        let mut last_angle = self.rotation;
        let choices_len = wheel_choices.len();

        for (i, choice) in wheel_choices.iter_mut().enumerate() {
            let angle_occupied = angle_step * choice.weight as f32;
            let start_angle = last_angle;
            let end_angle = start_angle + angle_occupied;
            last_angle = end_angle;

            let color = colors[(i + if choices_len % colors.len() == 1 && i + 1 == choices_len {
                1
            } else {
                0
            }) % colors.len()];

            let actual_steps = (constants::STEPS as u32 * choice.weight / total_weight) as u8;
            let points: Vec<Pos2> = (0..=actual_steps)
                .map(|j| {
                    let t = j as f32 / actual_steps as f32;
                    let angle = start_angle + t * (end_angle - start_angle);
                    egui::pos2(
                        self.center.x + self.radius * angle.cos(),
                        self.center.y + self.radius * angle.sin(),
                    )
                })
                .collect();

            let segment_width = points.first().unwrap().distance(*points.last().unwrap());

            painter.add(PathShape::convex_polygon(
                [&[self.center], points.as_slice(), &[self.center]].concat(),
                color,
                Stroke::NONE,
            ));

            let text_angle = start_angle + angle_occupied / 2.0;
            painter.add(Wheel::create_text_shape(
                choice.label.clone(),
                painter,
                text_angle,
                self.radius,
                self.center,
                segment_width,
            ));
        }
    }

    pub fn get_triangle_center(&self) -> Pos2 {
        egui::pos2(self.center.x + self.radius, self.center.y)
    }

    pub fn reset_rotation(&mut self, choices: &Vec<Choice>) {
        self.rotation = PI / choices.len() as f32
    }

    fn get_winner(&self, wheel_choices: &Vec<Choice>) -> Option<Choice> {
        if self.spinning {
            return None;
        }

        let angle_step = 2.0 * PI / Wheel::get_total_weight(wheel_choices) as f32;
        let mut last_angle = self.rotation;
        let mut minimum: Option<(Choice, f32)> = None;

        for choice in wheel_choices {
            let actual_end_angle = (last_angle + angle_step * choice.weight as f32) % (2.0 * PI);

            if minimum
                .as_ref()
                .map_or(true, |&(_, min_angle)| actual_end_angle < min_angle)
            {
                minimum = Some((choice.clone(), actual_end_angle));
            }

            last_angle += angle_step * choice.weight as f32;
        }

        minimum.map(|(choice, _)| choice)
    }

    fn create_text_shape(
        text: String,
        painter: &Painter,
        text_angle: f32,
        wheel_radius: f32,
        text_center: Pos2,
        segment_width: f32,
    ) -> TextShape {
        let actual_label = if text.len() > constants::MAX_RANGE_TEXT_LENGTH {
            format!("{}..", &text[..constants::MAX_RANGE_TEXT_LENGTH])
        } else {
            text
        };

        let text_radius = wheel_radius * 0.6;
        let real_width = segment_width.max(1.0);

        let mut current_text_size = constants::MAX_TEXT_SIZE;
        let galley = loop {
            let galley = painter.layout_no_wrap(
                actual_label.clone(),
                FontId::proportional(current_text_size as f32),
                Color32::WHITE,
            );
            let text_size = galley.size();
            if (text_size.x <= text_radius && text_size.y <= real_width * 0.9)
                || current_text_size <= constants::MIN_TEXT_SIZE
            {
                break galley;
            }
            current_text_size -= 1;
        };

        let text_center = Pos2::new(
            text_center.x + text_radius * text_angle.cos(),
            text_center.y + text_radius * text_angle.sin(),
        );

        let text_offset = Pos2::new(galley.size().x / 2.0, galley.size().y / 2.0);
        let rotated_offset = Pos2::new(
            text_offset.x * text_angle.cos() - text_offset.y * text_angle.sin(),
            text_offset.x * text_angle.sin() + text_offset.y * text_angle.cos(),
        );

        let centered_point = Pos2::new(
            text_center.x - rotated_offset.x,
            text_center.y - rotated_offset.y,
        );

        TextShape {
            angle: text_angle,
            ..TextShape::new(centered_point, galley, Color32::WHITE)
        }
    }

    fn get_total_weight(choices: &Vec<Choice>) -> u32 {
        choices.iter().map(|choice| choice.weight).sum()
    }
}
