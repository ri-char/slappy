use eframe::egui::{
    Color32, CornerRadius, CursorIcon, Label, Pos2, Rect, Response, Rgba, Sense, Slider, Stroke,
    StrokeKind, Ui, Vec2, Widget,
    color_picker::{Alpha, color_edit_button_rgba},
};

use crate::ui::{
    move_resize::{hover_range, key_arrow_to_offset},
    shape::{CreateAt, Shape},
    utils::{from_ratio_pos, to_ratio_pos, to_ratio_vec},
    window::RenderInfo,
};

#[derive(Clone)]
pub struct PenAttribute {
    pub line_width: f32,
    pub line_color: Rgba,
}

impl Default for PenAttribute {
    fn default() -> Self {
        Self {
            line_width: 3f32,
            line_color: Rgba::RED,
        }
    }
}

impl PenAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Line width").selectable(false).ui(ui);
        Slider::new(&mut self.line_width, 1f32..=20f32).ui(ui);
        ui.end_row();

        Label::new("Line Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.line_color, Alpha::OnlyBlend);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Pen {
    pub line: Vec<Pos2>,
    pub attributes: PenAttribute,
    drawing: bool,
    draw_finish: bool,
}
impl CreateAt for Pen {
    type Attr = PenAttribute;
    fn create_at(pos: Pos2, attributes: PenAttribute, render_info: &RenderInfo) -> Box<dyn Shape> {
        Box::new(Pen {
            line: vec![to_ratio_pos(&pos, &render_info.screenshot_rect)],
            attributes,
            drawing: true,
            draw_finish: false,
        })
    }
}

impl Shape for Pen {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        if self.line.len() <= 2 {
            return is_active;
        }
        let points: Vec<Pos2> = self
            .line
            .iter()
            .map(|p| from_ratio_pos(p, &render_info.screenshot_rect))
            .collect();
        let render_range = Rect::from_points(&points);
        let stroke = Stroke::new(
            self.attributes.line_width * render_info.pixel_ratio,
            self.attributes.line_color,
        );

        ui.painter().line(points, stroke);

        if is_active {
            if !self.drawing {
                let mut offset = Vec2::ZERO;
                if let Some(key_offset) = key_arrow_to_offset(ui) {
                    offset = key_offset;
                }

                ui.painter().rect_stroke(
                    render_range.expand(2f32),
                    CornerRadius::ZERO,
                    Stroke::new(1f32, Color32::from_gray(0xee)),
                    StrokeKind::Outside,
                );

                let resp = ui
                    .allocate_rect(render_range.expand(2f32), Sense::click_and_drag())
                    .on_hover_cursor(CursorIcon::Grab);
                if resp.dragged() {
                    ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                    offset = resp.drag_delta();
                }
                if offset != Vec2::ZERO {
                    let ratio_offset = to_ratio_vec(offset, &render_info.screenshot_rect);
                    for p in &mut self.line {
                        *p += ratio_offset;
                    }
                }
            }
            if self.draw_finish {
                self.draw_finish = false;
                false
            } else {
                true
            }
        } else {
            hover_range(ui, render_range.expand(2f32), render_info.shot_mode)
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui, _render_info: &RenderInfo) {
        self.attributes.ui(ui);
    }

    fn on_create_response(&mut self, _ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        if resp.dragged()
            && let Some(currnet_pos) = resp.interact_pointer_pos()
        {
            self.line
                .push(to_ratio_pos(&currnet_pos, &render_info.screenshot_rect));
        }
        if resp.drag_stopped() {
            self.drawing = false;
            self.draw_finish = true;
        }
    }
}
