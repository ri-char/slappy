use eframe::{
    egui::{
        Label, Pos2, Rect, Response, Rgba, Slider, Stroke, Ui, Vec2, Widget,
        color_picker::{Alpha, color_edit_button_rgba},
    },
    epaint::EllipseShape,
};

use crate::ui::{
    RenderInfo,
    move_resize::{MoveResize, ResizeMode, hover_range},
    shape::{CreateAt, Shape},
    utils::{from_ratio_rect, to_ratio_rect},
};

#[derive(Clone)]
pub struct CircleAttribute {
    pub line_width: f32,
    pub fill_color: Rgba,
    pub border_color: Rgba,
}

impl Default for CircleAttribute {
    fn default() -> Self {
        Self {
            line_width: 3f32,
            border_color: Rgba::RED,
            fill_color: Rgba::TRANSPARENT,
        }
    }
}

impl CircleAttribute {
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
    }
}

pub struct Circle {
    pub range: Rect,

    pub attributes: CircleAttribute,

    move_resize: MoveResize,
}

impl CreateAt for Circle {
    type Attr = CircleAttribute;
    fn create_at(pos: Pos2, attr: CircleAttribute, render_info: &RenderInfo) -> Box<dyn Shape> {
        Box::new(Circle {
            range: to_ratio_rect(
                &Rect::from_min_max(pos, pos + Vec2::splat(30f32)),
                &render_info.screenshot_rect,
            ),
            attributes: attr,
            move_resize: MoveResize::resize(pos),
        })
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
            hover_range(
                ui,
                render_range.expand(self.attributes.line_width / 2f32 + 2f32),
                render_info.shot_mode,
            )
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui) {
        self.attributes.ui(ui);
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.move_resize
            .handle_resize(ui, resp, render_info, &mut self.range, ResizeMode::None);
    }
}
