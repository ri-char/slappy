use std::sync::atomic::AtomicU32;

use eframe::epaint::PathShape;
use eframe::{
    egui::{
        Align2, DragValue, FontFamily, FontId, Label, Pos2, Rect, Response, Rgba, Sense, Slider,
        Stroke, Ui, Vec2, Widget,
        color_picker::{Alpha, color_edit_button_rgba},
        emath::Rot2,
    },
    epaint::PathStroke,
};

use crate::{
    ui::{RenderInfo, move_resize::LineMove, shape::Shape},
    utils::{from_ratio_pos, to_ratio_pos},
};

#[derive(Clone)]
pub struct NumberAttribute {
    pub fill_color: Rgba,
    pub text_color: Rgba,
    pub circle_size: f32,
    pub font_size: f32,
}

impl Default for NumberAttribute {
    fn default() -> Self {
        Self {
            fill_color: Rgba::RED,
            text_color: Rgba::WHITE,
            circle_size: 25f32,
            font_size: 15f32,
        }
    }
}

impl NumberAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.fill_color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Text Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.text_color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Circle size").selectable(false).ui(ui);
        Slider::new(&mut self.circle_size, 0f32..=100f32).ui(ui);
        ui.end_row();

        Label::new("Font size").selectable(false).ui(ui);
        Slider::new(&mut self.font_size, 0f32..=50f32).ui(ui);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Number {
    pub start_pos: Pos2,
    pub end_pos: Pos2,

    pub attributes: NumberAttribute,
    pub number: u32,

    line_move: LineMove,
}

impl Number {
    pub fn create_at(pos: Pos2, attributes: NumberAttribute, render_info: &RenderInfo) -> Self {
        static INC_NUMBER_BITMAP: AtomicU32 = AtomicU32::new(1);
        let n = INC_NUMBER_BITMAP.fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        Number {
            start_pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            end_pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            attributes,
            line_move: Default::default(),
            number: n,
        }
    }
}

impl Shape for Number {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_start_pos = from_ratio_pos(&self.start_pos, &render_info.screenshot_rect);
        let render_end_pos = from_ratio_pos(&self.end_pos, &render_info.screenshot_rect);

        let circle_radius = self.attributes.circle_size * render_info.pixel_ratio;

        let vec = render_end_pos - render_start_pos;
        if vec.length() >= circle_radius {
            let rot = Rot2::from_angle(std::f32::consts::TAU / 15.0);
            let vec = vec.normalized();
            let pos1 = render_start_pos + circle_radius * (rot * vec);
            let pos2 = render_start_pos + circle_radius * (rot.inverse() * vec);
            ui.painter().add(PathShape {
                points: vec![pos1, render_end_pos, pos2],
                closed: true,
                fill: self.attributes.fill_color.into(),
                stroke: PathStroke::NONE,
            });
        }
        ui.painter().circle(
            render_start_pos,
            circle_radius,
            self.attributes.fill_color,
            Stroke::NONE,
        );
        ui.painter().text(
            render_start_pos,
            Align2::CENTER_CENTER,
            self.number,
            FontId::new(self.attributes.font_size, FontFamily::Proportional),
            self.attributes.text_color.into(),
        );

        if is_active {
            self.line_move.ui(
                ui,
                render_info,
                &mut self.start_pos,
                &mut self.end_pos,
                circle_radius,
            );
            true
        } else {
            let mut render_range =
                Rect::from_center_size(render_start_pos, Vec2::splat(circle_radius * 2f32));
            render_range.extend_with(render_end_pos);
            let render_range = render_range.expand(2f32);
            let resp = ui.allocate_rect(render_range, Sense::click());

            resp.clicked()
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui) {
        self.attributes.ui(ui);

        Label::new("Number").selectable(false).ui(ui);
        DragValue::new(&mut self.number).range(0u32..=128u32).ui(ui);
        ui.end_row();
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        self.line_move
            .handle_move_end(ui, resp, render_info, &self.start_pos, &mut self.end_pos);
    }
}
