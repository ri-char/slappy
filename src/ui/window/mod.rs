use eframe::egui::{FontFamily, Rect};

pub mod edit_window;
pub mod pin_window;

pub struct RenderInfo {
    pub screenshot_rect: Rect,
    pub pixel_ratio: f32,
    pub user_font: FontFamily,
    pub shot_mode: bool,
}
