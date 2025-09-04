use eframe::egui::{
    Align2, Color32, ComboBox, CornerRadius, CursorIcon, FontFamily, FontId, Label, Pos2, Response,
    Rgba, Sense, Slider, Stroke, StrokeKind, TextEdit, Ui, Widget,
    color_picker::{Alpha, color_edit_button_rgba},
};

use crate::{
    ui::{RenderInfo, shape::Shape},
    utils::{from_ratio_pos, to_ratio_pos},
};

#[derive(Clone)]
pub struct TextAttribute {
    color: Rgba,
    text: String,
    family: FontFamily,
    size: f32,
}

impl Default for TextAttribute {
    fn default() -> Self {
        Self {
            color: Rgba::RED,
            text: "Edit Text here".to_string(),
            size: 20f32,
            family: FontFamily::Proportional,
        }
    }
}

impl TextAttribute {
    pub fn ui(&mut self, ui: &mut Ui) {
        Label::new("Text Color").selectable(false).ui(ui);
        color_edit_button_rgba(ui, &mut self.color, Alpha::OnlyBlend);
        ui.end_row();

        Label::new("Text").selectable(false).ui(ui);
        TextEdit::multiline(&mut self.text).ui(ui);
        ui.end_row();

        Label::new("Font Family").selectable(false).ui(ui);
        ComboBox::from_id_salt("font_family")
            .selected_text(format!("{}", self.family))
            .show_ui(ui, |ui| {
                for f in ui
                    .ctx()
                    .fonts(|f| f.families().iter().cloned().collect::<Vec<_>>())
                {
                    ui.selectable_value(&mut self.family, f.clone(), f.to_string());
                }
            });

        ui.end_row();

        Label::new("Font Size").selectable(false).ui(ui);
        Slider::new(&mut self.size, 0f32..=100f32).ui(ui);
        ui.end_row();
    }
}

#[derive(Clone)]
pub struct Text {
    pub pos: Pos2,

    pub attributes: TextAttribute,
}

impl Text {
    pub fn create_at(pos: Pos2, mut attributes: TextAttribute, render_info: &RenderInfo) -> Self {
        if attributes.text.trim().is_empty() {
            attributes.text = "Edit Text here".to_string();
        }
        Text {
            pos: to_ratio_pos(&pos, &render_info.screenshot_rect),
            attributes,
        }
    }
}

impl Shape for Text {
    fn ui(&mut self, ui: &mut Ui, is_active: bool, render_info: &RenderInfo) -> bool {
        let render_pos = from_ratio_pos(&self.pos, &render_info.screenshot_rect);
        let render_range = ui.painter().text(
            render_pos,
            Align2::CENTER_CENTER,
            self.attributes.text.as_str(),
            FontId::new(self.attributes.size, self.attributes.family.clone()),
            self.attributes.color.into(),
        );

        if is_active {
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
            let response = ui.allocate_rect(render_range.expand(2f32), Sense::click());
            response.clicked()
        }
    }

    fn toolbar_ui(&mut self, ui: &mut Ui) {
        self.attributes.ui(ui);
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
