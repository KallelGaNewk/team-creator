use eframe::egui;
use eframe::egui::Context;

pub trait StringExt {
    /// Return `default` if the string is empty, otherwise return the string as `&str`.
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str;
}

impl StringExt for str {
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        if self.is_empty() { default } else { self }
    }
}

impl StringExt for String {
    fn as_str_or<'a>(&'a self, default: &'a str) -> &'a str {
        self.as_str().as_str_or(default)
    }
}

pub trait PressedEnterExt {
    /// Returns `true` if the Enter key was pressed while the response lost focus.
    fn pressed_enter(&self, ctx: &Context) -> bool;
}

impl PressedEnterExt for egui::Response {
    fn pressed_enter(&self, ctx: &Context) -> bool {
        self.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))
    }
}
