use eframe::{
    egui::{Pos2, Rect, Response, Sense, Stroke, Ui},
    epaint::EllipseShape,
};

use crate::{
    ui::{
        GeneralAttribute, RenderInfo,
        move_resize::{MoveResize, ResizeMode},
        shape::Shape,
    },
    utils::from_ratio_rect,
};

pub struct Circle {
    pub range: Rect,

    pub attributes: GeneralAttribute,

    move_resize: MoveResize,
}

impl Circle {
    pub fn create_at(pos: Pos2, attr: GeneralAttribute) -> Self {
        Circle {
            range: Rect::ZERO,
            attributes: attr,
            move_resize: MoveResize::resize(pos),
        }
    }
}

impl Shape for Circle {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_range = from_ratio_rect(&self.range, &render_info.screenshot_rect);
        ui.painter().add(EllipseShape {
            center: render_range.center(),
            radius: render_range.size() / 2f32,
            fill: self.attributes.fill_color.into(),
            stroke: Stroke::new(
                self.attributes.line_width * render_info.pixel_ratio,
                self.attributes.border_color,
            ),
        });
        if is_active {
            self.move_resize.ui(ui, render_info, &mut self.range);
            true
        } else {
            let response = ui.allocate_rect(render_range, Sense::click());
            response.clicked()
        }
    }

    fn has_toolbar(&self) -> bool {
        true
    }

    fn toolbar_ui(&mut self, ui: &mut Ui) {
        self.attributes.ui(ui, |_| {});
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.move_resize
            .handle_resize(ui, resp, render_info, &mut self.range, ResizeMode::None);
    }
}
