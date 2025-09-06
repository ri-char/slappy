use std::sync::atomic::AtomicU32;

use eframe::egui::{Pos2, Response, Ui, ahash::HashMap};

use crate::ui::RenderInfo;
pub mod circle;
pub mod line;
pub mod number;
pub mod pen;
pub mod rectangle;
pub mod text;

pub trait Shape {
    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo);
    /// draw ui, and return `true` if it is actived
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool;
    fn toolbar_ui(&mut self, ui: &mut Ui, render_info: &RenderInfo);
}

pub trait CreateAt: Shape {
    type Attr: Clone;
    fn create_at(pos: Pos2, attr: Self::Attr, render_info: &RenderInfo) -> Box<dyn Shape>;

    fn handle_create_response(
        ui: &mut Ui,
        resp: &eframe::egui::Response,
        render_info: &RenderInfo,
        attr: &Self::Attr,
        active_shape_id: &mut Option<ShapeId>,
        shapes: &mut HashMap<ShapeId, Box<dyn Shape>>,
    ) {
        if resp.clicked()
            && let Some(pos) = resp.interact_pointer_pos()
        {
            if active_shape_id.is_some() {
                *active_shape_id = None;
            } else {
                let shape_id = ShapeId::new();
                shapes.insert(shape_id, Self::create_at(pos, attr.clone(), render_info));
                *active_shape_id = Some(shape_id);
            }
        }
        if resp.drag_started()
            && let Some(pos) = resp.interact_pointer_pos()
        {
            let shape_id = ShapeId::new();
            shapes.insert(shape_id, Self::create_at(pos, attr.clone(), render_info));
            *active_shape_id = Some(shape_id);
        } else if (resp.drag_stopped() || resp.dragged())
            && let Some(active_shape) = active_shape_id.and_then(|id| shapes.get_mut(&id))
        {
            active_shape.on_create_response(ui, resp, render_info);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ShapeId(u32);

impl ShapeId {
    pub fn new() -> Self {
        static AUTO_INC_ID: AtomicU32 = AtomicU32::new(0);
        Self(AUTO_INC_ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel))
    }
}
