use eframe::egui::{
    CornerRadius, Label, Pos2, Rect, Response, Rgba, Slider, Stroke, StrokeKind, Ui, Vec2, Widget,
    color_picker::{Alpha, color_edit_button_rgba},
};

use crate::ui::{
    move_resize::{MoveResize, ResizeMode, hover_range},
    shape::{CreateAt, Shape},
    utils::{from_ratio_rect, to_ratio_rect},
    window::RenderInfo,
};

#[derive(Clone)]
pub struct RectangleAttribute {
    pub line_width: f32,
    pub fill_color: Rgba,
    pub border_color: Rgba,
    pub radius: f32,
}

impl Default for RectangleAttribute {
    fn default() -> Self {
        Self {
            line_width: 3f32,
            border_color: Rgba::RED,
            fill_color: Rgba::TRANSPARENT,
            radius: 0f32,
        }
    }
}

impl RectangleAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Line width").selectable(false).ui(ui);
        Slider::new(&mut self.line_width, 1f32..=20f32).ui(ui);
        ui.end_row();

        Label::new("Fill Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.fill_color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Border Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.border_color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Radius").selectable(false).ui(ui);
        Slider::new(&mut self.radius, 0f32..=1f32).ui(ui);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub range: Rect,

    pub attributes: RectangleAttribute,

    move_resize: MoveResize,
}

impl CreateAt for Rectangle {
    type Attr = RectangleAttribute;
    fn create_at(
        pos: Pos2,
        attributes: RectangleAttribute,
        render_info: &RenderInfo,
    ) -> Box<dyn Shape> {
        Box::new(Rectangle {
            range: to_ratio_rect(
                &Rect::from_min_max(pos, pos + Vec2::splat(30f32)),
                &render_info.screenshot_rect,
            ),
            move_resize: MoveResize::resize(pos),
            attributes,
        })
    }
}

impl Shape for Rectangle {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_range = from_ratio_rect(&self.range, &render_info.screenshot_rect);
        ui.painter().rect(
            render_range,
            CornerRadius::same(
                (self.attributes.radius * render_range.width().min(render_range.height())) as u8,
            ),
            self.attributes.fill_color,
            Stroke::new(
                self.attributes.line_width * render_info.pixel_ratio,
                self.attributes.border_color,
            ),
            StrokeKind::Middle,
        );
        if is_active {
            self.move_resize.ui(ui, render_info, &mut self.range);
            true
        } else {
            hover_range(
                ui,
                render_range.expand(self.attributes.line_width / 2f32 + 2f32),
                render_info.shot_mode,
            )
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui, _render_info: &RenderInfo) {
        self.attributes.ui(ui);
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.move_resize
            .handle_resize(ui, resp, render_info, &mut self.range, ResizeMode::None);
    }
}
