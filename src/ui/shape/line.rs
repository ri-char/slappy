use eframe::egui::emath::Rot2;
use eframe::egui::{
    Checkbox, CursorIcon, Label, Pos2, Rect, Response, Sense, Slider, Stroke, Ui, Widget,
};
use eframe::epaint::PathShape;

use crate::{
    ui::{GeneralAttribute, RenderInfo, move_resize::add_control_point, shape::Shape},
    utils::{from_ratio_pos, to_ratio_pos},
};

#[derive(Clone)]
pub struct LineAttribute {
    arrow_start: bool,
    arrow_end: bool,
    arrow_size: f32,
}

impl Default for LineAttribute {
    fn default() -> Self {
        Self {
            arrow_start: false,
            arrow_end: false,
            arrow_size: 15f32,
        }
    }
}

impl LineAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
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

    pub general_attributes: GeneralAttribute,
    pub line_attributes: LineAttribute,
}

impl Line {
    pub fn create_at(
        pos: Pos2,
        general_attributes: GeneralAttribute,
        line_attributes: LineAttribute,
        render_info: &RenderInfo,
    ) -> Self {
        Line {
            start_pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            end_pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            general_attributes,
            line_attributes,
        }
    }
}

fn calc_pos_with_shfit_modifier(
    ui: &mut Ui,
    pos: Pos2,
    ratio_base: &Pos2,
    render_info: &RenderInfo,
) -> Pos2 {
    if ui.input(|i| i.modifiers.shift) {
        let base = from_ratio_pos(ratio_base, &render_info.screenshot_rect);
        let offset = (pos - base).abs();
        if offset.x < offset.y {
            Pos2 {
                x: base.x,
                y: pos.y,
            }
        } else {
            Pos2 {
                x: pos.x,
                y: base.y,
            }
        }
    } else {
        pos
    }
}

impl Line {
    fn render_arrow(ui: &mut Ui, start_pos: Pos2, end_pos: Pos2, stroke: Stroke, size: f32) {
        let rot = Rot2::from_angle(std::f32::consts::TAU / 15.0);
        let vec = end_pos - start_pos;

        let dir = vec.normalized();
        let pos1 = end_pos - size * (rot.inverse() * dir);
        let pos2 = end_pos - size * (rot * dir);
        let pos3 = end_pos - size * 0.7f32 * dir;
        ui.painter().add(PathShape {
            points: vec![pos1, end_pos, pos2, pos3],
            closed: true,
            fill: stroke.color,
            stroke: stroke.into(),
        });
    }
}

impl Shape for Line {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_start_pos = from_ratio_pos(&self.start_pos, &render_info.screenshot_rect);
        let render_end_pos = from_ratio_pos(&self.end_pos, &render_info.screenshot_rect);

        let stroke = Stroke::new(
            self.general_attributes.line_width * render_info.pixel_ratio,
            self.general_attributes.border_color,
        );

        ui.painter()
            .line_segment([render_start_pos, render_end_pos], stroke);
        if self.line_attributes.arrow_end {
            let tip_length = self.line_attributes.arrow_size * render_info.pixel_ratio;
            Line::render_arrow(ui, render_start_pos, render_end_pos, stroke, tip_length);
        }

        if self.line_attributes.arrow_start {
            let tip_length = self.line_attributes.arrow_size * render_info.pixel_ratio;
            Line::render_arrow(ui, render_end_pos, render_start_pos, stroke, tip_length);
        }

        if is_active {
            let handle = add_control_point(ui, render_start_pos, CursorIcon::Grab);
            if handle.dragged()
                && let Some(current_pos) = handle.interact_pointer_pos()
            {
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                self.start_pos = to_ratio_pos(
                    &calc_pos_with_shfit_modifier(ui, current_pos, &self.end_pos, render_info),
                    &render_info.screenshot_rect,
                );
            }

            let handle = add_control_point(ui, render_end_pos, CursorIcon::Grab);
            if handle.dragged()
                && let Some(current_pos) = handle.interact_pointer_pos()
            {
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                self.end_pos = to_ratio_pos(
                    &calc_pos_with_shfit_modifier(ui, current_pos, &self.start_pos, render_info),
                    &render_info.screenshot_rect,
                );
            }

            true
        } else {
            let response = ui.allocate_rect(
                Rect::from_two_pos(render_start_pos, render_end_pos),
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
            self.line_attributes.ui(ui);
        });
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        if resp.dragged()
            && let Some(current_pos) = resp.interact_pointer_pos()
        {
            self.end_pos = to_ratio_pos(
                &calc_pos_with_shfit_modifier(ui, current_pos, &self.start_pos, render_info),
                &render_info.screenshot_rect,
            );
        }
    }
}
