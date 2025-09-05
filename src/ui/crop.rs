use eframe::egui::{Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui};

use crate::{
    ui::utils::from_ratio_rect,
    ui::{RenderInfo, move_resize::MoveResize},
};

const FULL_RECT: Rect = Rect {
    min: Pos2 { x: 0f32, y: 0f32 },
    max: Pos2 { x: 1f32, y: 1f32 },
};

#[derive(Clone, Debug)]
pub struct CropTool {
    pub cropped_range: Rect,
    move_resize: MoveResize,
}

impl Default for CropTool {
    fn default() -> Self {
        Self {
            cropped_range: FULL_RECT,
            move_resize: Default::default(),
        }
    }
}

impl CropTool {
    pub fn on_global_response(
        &mut self,
        ui: &mut Ui,
        resp: &eframe::egui::Response,
        render_info: &RenderInfo,
    ) {
        if resp.clicked()
            && let Some(clicked_pos) = resp.interact_pointer_pos()
            && !from_ratio_rect(&self.cropped_range, &render_info.screenshot_rect)
                .contains(clicked_pos)
        {
            self.cropped_range = FULL_RECT;
        }
        self.move_resize.handle_resize(
            ui,
            resp,
            render_info,
            &mut self.cropped_range,
            super::move_resize::ResizeMode::Cursor,
        );
    }

    pub fn ui(&mut self, ui: &mut Ui, render_info: &RenderInfo, active: bool) {
        if self.cropped_range == FULL_RECT {
            return;
        }
        const STROKE_COLOR: Color32 = Color32::from_gray(0xee);
        const FILL_COLOR: Color32 = Color32::from_rgba_premultiplied(0x30, 0x30, 0x30, 0x90);
        let stroke = Stroke {
            width: 2.0,
            color: STROKE_COLOR,
        };
        let render_range = from_ratio_rect(&self.cropped_range, &render_info.screenshot_rect);

        // draw the shadow
        ui.painter().rect_filled(
            Rect {
                min: render_info.screenshot_rect.min,
                max: Pos2 {
                    x: render_info.screenshot_rect.right(),
                    y: render_range.top(),
                },
            },
            CornerRadius::ZERO,
            FILL_COLOR,
        );

        ui.painter().rect_filled(
            Rect {
                min: Pos2 {
                    x: render_info.screenshot_rect.left(),
                    y: render_range.top(),
                },
                max: render_range.left_bottom(),
            },
            CornerRadius::ZERO,
            FILL_COLOR,
        );
        ui.painter().rect_filled(
            Rect {
                min: render_range.right_top(),
                max: Pos2 {
                    x: render_info.screenshot_rect.right(),
                    y: render_range.bottom(),
                },
            },
            CornerRadius::ZERO,
            FILL_COLOR,
        );
        ui.painter().rect_filled(
            Rect {
                min: Pos2 {
                    x: render_info.screenshot_rect.left(),
                    y: render_range.bottom(),
                },
                max: render_info.screenshot_rect.max,
            },
            CornerRadius::ZERO,
            FILL_COLOR,
        );
        // draw the border
        ui.painter().rect_stroke(
            render_range,
            CornerRadius::ZERO,
            stroke,
            StrokeKind::Outside,
        );
        // draw the handles
        if active {
            self.move_resize
                .ui(ui, render_info, &mut self.cropped_range);
        }
    }
}
