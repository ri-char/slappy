use std::sync::atomic::AtomicU32;

use eframe::egui::{Response, Ui};

use crate::ui::RenderInfo;
pub mod circle;
pub mod line;
pub mod rectangle;

pub trait Shape {
    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo);
    /// draw ui, and return `true` if it is actived
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool;
    fn has_toolbar(&self) -> bool;
    fn toolbar_ui(&mut self, ui: &mut Ui);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ShapeId(u32);

impl ShapeId {
    pub fn new() -> Self {
        static AUTO_INC_ID: AtomicU32 = AtomicU32::new(0);
        Self(AUTO_INC_ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel))
    }
}
