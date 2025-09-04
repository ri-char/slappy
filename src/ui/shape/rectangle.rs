use eframe::egui::{
    CornerRadius, Label, Pos2, Rect, Response, Sense, Slider, Stroke, StrokeKind, Ui, Widget,
};

use crate::{
    ui::{
        GeneralAttribute, RenderInfo,
        move_resize::{MoveResize, ResizeMode},
        shape::Shape,
    },
    utils::from_ratio_rect,
};

#[derive(Default, Clone)]
pub struct RectangleAttribute {
    radius: f32,
}

impl RectangleAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Radius").selectable(false).ui(ui);
        Slider::new(&mut self.radius, 0f32..=1f32).ui(ui);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub range: Rect,

    pub general_attributes: GeneralAttribute,
    pub rect_attributes: RectangleAttribute,

    move_resize: MoveResize,
}

impl Rectangle {
    pub fn create_at(
        pos: Pos2,
        general_attributes: GeneralAttribute,
        rect_attributes: RectangleAttribute,
    ) -> Self {
        Rectangle {
            range: Rect::ZERO,
            general_attributes,
            move_resize: MoveResize::resize(pos),
            rect_attributes,
        }
    }
}

impl Shape for Rectangle {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_range = from_ratio_rect(&self.range, &render_info.screenshot_rect);
        ui.painter().rect(
            render_range,
            CornerRadius::same(
                (self.rect_attributes.radius * render_range.width().min(render_range.height()))
                    as u8,
            ),
            self.general_attributes.fill_color,
            Stroke::new(
                self.general_attributes.line_width * render_info.pixel_ratio,
                self.general_attributes.border_color,
            ),
            StrokeKind::Middle,
        );
        if is_active {
            self.move_resize.ui(ui, render_info, &mut self.range);
            true
        } else {
            let response = ui.allocate_rect(
                render_range.expand(self.general_attributes.line_width / 2f32),
                Sense::click(),
            );
            response.clicked()
        }
    }

    fn has_toolbar(&self) -> bool {
        true
    }

    fn toolbar_ui(&mut self, ui: &mut Ui) {
        self.general_attributes.ui(ui, |ui| {
            self.rect_attributes.ui(ui);
        });
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.move_resize
            .handle_resize(ui, resp, render_info, &mut self.range, ResizeMode::None);
    }
}
