use eframe::egui::{
    color_picker::{color_edit_button_rgba, Alpha}, Align2, Color32, CornerRadius, CursorIcon, FontFamily, FontId, FontSelection, Label, Pos2, Response, Rgba, Sense, Slider, Stroke, StrokeKind, TextEdit, Ui, Widget
};

use crate::ui::{
    RenderInfo,
    move_resize::{hover_range, key_arrow_to_offset},
    shape::{CreateAt, Shape},
    utils::{from_ratio_pos, to_ratio_pos},
};

#[derive(Clone)]
pub struct TextAttribute {
    color: Rgba,
    text: String,
    size: f32,
}

impl Default for TextAttribute {
    fn default() -> Self {
        Self {
            color: Rgba::RED,
            text: "Edit Text here".to_string(),
            size: 20f32,
        }
    }
}

impl TextAttribute {
    pub fn ui(&mut self, ui: &mut Ui, font_family: FontFamily) {
        Label::new("Text Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Text").selectable(false).ui(ui);
        TextEdit::multiline(&mut self.text).font(FontSelection::FontId(FontId::new(12f32, font_family))).ui(ui);
        ui.end_row();

        Label::new("Font Size").selectable(false).ui(ui);
        Slider::new(&mut self.size, 0f32..=200f32).ui(ui);
        ui.end_row();
    }
}

pub struct Text {
    pub pos: Pos2,

    pub attributes: TextAttribute,
}


impl CreateAt for Text {
    type Attr = TextAttribute;
    fn create_at(
        pos: Pos2,
        mut attributes: TextAttribute,
        render_info: &RenderInfo,
    ) -> Box<dyn Shape> {
        if attributes.text.trim().is_empty() {
            attributes.text = "Edit Text here".to_string();
        }
        Box::new(Text {
            pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            attributes,
        })
    }
}

impl Shape for Text {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_pos = from_ratio_pos(&self.pos, &render_info.screenshot_rect);
        
        let render_range = ui.painter().text(
            render_pos,
            Align2::CENTER_CENTER,
            self.attributes.text.as_str(),
            FontId::new(self.attributes.size, render_info.user_font.clone()),
            self.attributes.color.into(),
        );

        if is_active {
            if let Some(offset) = key_arrow_to_offset(ui) {
                self.pos = to_ratio_pos(&(render_pos + offset), &render_info.screenshot_rect);
            }

            ui.painter().rect_stroke(
                render_range.expand(2f32),
                CornerRadius::ZERO,
                Stroke::new(1f32, Color32::from_gray(0xee)),
                StrokeKind::Outside,
            );

            let response = ui
                .allocate_rect(render_range.expand(2f32), Sense::click_and_drag())
                .on_hover_cursor(CursorIcon::Grab);
            self.on_create_response(ui, &response, render_info);
            true
        } else {
            hover_range(ui, render_range.expand(2f32), render_info.shot_mode)
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui, render_info: &RenderInfo) {
        self.attributes.ui(ui, render_info.user_font.clone());
    }

    fn on_create_response(&mut self, ui: &mut Ui, resp: &Response, render_info: &RenderInfo) {
        if resp.dragged() {
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            self.pos = to_ratio_pos(
                &(from_ratio_pos(&self.pos, &render_info.screenshot_rect) + resp.drag_delta()),
                &render_info.screenshot_rect,
            );
        }
    }
}
