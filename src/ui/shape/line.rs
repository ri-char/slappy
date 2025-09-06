use eframe::egui::{
    Checkbox, Label, Pos2, Rect, Response, Rgba, Slider, Stroke, Ui, Vec2, Widget,
    color_picker::{Alpha, color_edit_button_rgba},
    emath::Rot2,
};
use eframe::epaint::PathShape;

use crate::ui::{
    RenderInfo,
    move_resize::{LineMove, hover_range},
    shape::{CreateAt, Shape},
    utils::{from_ratio_pos, to_ratio_pos},
};

#[derive(Clone)]
pub struct LineAttribute {
    pub line_width: f32,
    pub line_color: Rgba,
    pub arrow_start: bool,
    pub arrow_end: bool,
    pub arrow_size: f32,
}

impl Default for LineAttribute {
    fn default() -> Self {
        Self {
            line_width: 3f32,
            line_color: Rgba::RED,
            arrow_start: false,
            arrow_end: false,
            arrow_size: 15f32,
        }
    }
}

impl LineAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Line width").selectable(false).ui(ui);
        Slider::new(&mut self.line_width, 1f32..=20f32).ui(ui);
        ui.end_row();

        Label::new("Line Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.line_color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Arrow at start").selectable(false).ui(ui);
        Checkbox::new(&mut self.arrow_start, "Arrow at start").ui(ui);
        ui.end_row();

        Label::new("Arrow at end").selectable(false).ui(ui);
        Checkbox::new(&mut self.arrow_end, "Arrow at end").ui(ui);
        ui.end_row();

        Label::new("Arrow size").selectable(false).ui(ui);
        Slider::new(&mut self.arrow_size, 0f32..=50f32).ui(ui);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Line {
    pub start_pos: Pos2,
    pub end_pos: Pos2,

    pub attributes: LineAttribute,

    line_move: LineMove,
}

impl CreateAt for Line {
    type Attr = LineAttribute;
    fn create_at(pos: Pos2, attributes: LineAttribute, render_info: &RenderInfo) -> Box<dyn Shape> {
        Box::new(Line {
            start_pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            end_pos: to_ratio_pos(
                &(pos + Vec2 { x: 30f32, y: 0f32 }),
                &render_info.screenshot_rect,
            ),
            attributes,
            line_move: Default::default(),
        })
    }
}

impl Shape for Line {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_start_pos = from_ratio_pos(&self.start_pos, &render_info.screenshot_rect);
        let render_end_pos = from_ratio_pos(&self.end_pos, &render_info.screenshot_rect);

        let stroke = Stroke::new(
            self.attributes.line_width * render_info.pixel_ratio,
            self.attributes.line_color,
        );

        ui.painter()
            .line_segment([render_start_pos, render_end_pos], stroke);

        let tip_length = self.attributes.arrow_size * render_info.pixel_ratio;
        let mut render_range = Rect::from_two_pos(render_start_pos, render_end_pos);
        let mut render_arrow = |start_pos: Pos2, end_pos: Pos2| {
            let rot = Rot2::from_angle(std::f32::consts::TAU / 15.0);
            let vec = end_pos - start_pos;

            let dir = vec.normalized();
            let pos1 = end_pos - tip_length * (rot.inverse() * dir);
            let pos2 = end_pos - tip_length * (rot * dir);
            let pos3 = end_pos - tip_length * 0.7f32 * dir;
            render_range.extend_with(pos1);
            render_range.extend_with(pos2);
            ui.painter().add(PathShape {
                points: vec![pos1, end_pos, pos2, pos3],
                closed: true,
                fill: stroke.color,
                stroke: stroke.into(),
            });
        };

        if self.attributes.arrow_end {
            render_arrow(render_start_pos, render_end_pos);
        }

        if self.attributes.arrow_start {
            render_arrow(render_end_pos, render_start_pos);
        }

        if is_active {
            self.line_move.ui(
                ui,
                render_info,
                &mut self.start_pos,
                &mut self.end_pos,
                0f32,
            );
            true
        } else {
            hover_range(ui, render_range.expand(2f32), render_info.shot_mode)
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui, _render_info: &RenderInfo) {
        self.attributes.ui(ui);
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.line_move
            .handle_move_end(ui, resp, render_info, &self.start_pos, &mut self.end_pos);
    }
}
