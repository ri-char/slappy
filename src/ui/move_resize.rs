use eframe::egui::{
    Color32, CursorIcon, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::{
    ui::RenderInfo,
    utils::{from_ratio_rect, to_ratio_rect},
};

#[derive(Debug, Default, Clone)]
enum MoveResizeState {
    #[default]
    None,
    Move {
        offset: Vec2,
        size: Vec2,
    },
    Resize {
        fixed_pos: Pos2,
    },
    ResizeSide(ResizeSideInfo),
}

#[derive(Debug, Default, Clone)]
pub struct ResizeSideInfo {
    pub fixed: Pos2,
    pub length: f32,
    pub is_x: bool,
}

#[derive(Debug, Clone)]
pub enum ResizeMode {
    /// can not start a new resize operator
    None,
    /// the fixed point is given
    Fixed(Pos2),
    /// start from cursor
    Cursor,
}

#[derive(Debug, Default, Clone)]
pub struct MoveResize {
    state: MoveResizeState,
}

impl MoveResize {
    #[inline]
    pub const fn resize(start_pos: Pos2) -> Self {
        MoveResize {
            state: MoveResizeState::Resize {
                fixed_pos: start_pos,
            },
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, render_info: &RenderInfo, rect: &mut Rect) {
        let render_range = from_ratio_rect(rect, &render_info.screenshot_rect);
        let resp = ui
            .allocate_rect(render_range, Sense::drag())
            .on_hover_cursor(CursorIcon::Grab);
        self.handle_move(ui, &resp, render_info, rect);

        for (move_pos, fixed_pos, cursor_icon) in [
            (
                render_range.left_top(),
                render_range.right_bottom(),
                CursorIcon::ResizeNorthWest,
            ),
            (
                render_range.right_bottom(),
                render_range.left_top(),
                CursorIcon::ResizeSouthEast,
            ),
            (
                render_range.right_top(),
                render_range.left_bottom(),
                CursorIcon::ResizeNorthEast,
            ),
            (
                render_range.left_bottom(),
                render_range.right_top(),
                CursorIcon::ResizeSouthWest,
            ),
        ] {
            let handle = add_control_point(ui, move_pos, cursor_icon);
            self.handle_resize(ui, &handle, render_info, rect, ResizeMode::Fixed(fixed_pos));
        }

        let handle = add_control_point(ui, render_range.left_center(), CursorIcon::ResizeWest);
        self.handle_resize_side(&handle, render_info, rect, || ResizeSideInfo {
            fixed: render_range.right_top(),
            length: render_range.height(),
            is_x: true,
        });

        let handle = add_control_point(ui, render_range.right_center(), CursorIcon::ResizeEast);
        self.handle_resize_side(&handle, render_info, rect, || ResizeSideInfo {
            fixed: render_range.left_top(),
            length: render_range.height(),
            is_x: true,
        });

        let handle = add_control_point(ui, render_range.center_top(), CursorIcon::ResizeNorth);
        self.handle_resize_side(&handle, render_info, rect, || ResizeSideInfo {
            fixed: render_range.left_bottom(),
            length: render_range.width(),
            is_x: false,
        });

        let handle = add_control_point(ui, render_range.center_bottom(), CursorIcon::ResizeSouth);
        self.handle_resize_side(&handle, render_info, rect, || ResizeSideInfo {
            fixed: render_range.left_top(),
            length: render_range.width(),
            is_x: false,
        });
    }

    /// handle resize event.
    pub fn handle_resize(
        &mut self,
        ui: &mut Ui,
        resp: &Response,
        render_info: &RenderInfo,
        rect: &mut Rect,
        mode: ResizeMode,
    ) {
        if resp.dragged() {
            if resp.drag_started() {
                match mode {
                    ResizeMode::Fixed(fixed_pos) => {
                        self.state = MoveResizeState::Resize { fixed_pos };
                    }
                    ResizeMode::Cursor => {
                        if let Some(fixed_pos) = resp.interact_pointer_pos() {
                            self.state = MoveResizeState::Resize { fixed_pos };
                        }
                    }
                    ResizeMode::None => {}
                }
            }
            if let Some(pos) = resp.interact_pointer_pos()
                && let MoveResizeState::Resize { fixed_pos } = self.state
            {
                let new_range = if ui.input(|i| i.modifiers.shift) {
                    let offset = pos - fixed_pos;
                    let offset_abs = offset.abs();
                    let size = f32::min(offset_abs.x, offset_abs.y);
                    Rect::from_two_pos(
                        Pos2 {
                            x: fixed_pos.x + size.copysign(offset.x),
                            y: fixed_pos.y + size.copysign(offset.y),
                        },
                        fixed_pos,
                    )
                } else {
                    Rect::from_two_pos(pos, fixed_pos)
                };
                *rect = to_ratio_rect(&new_range, &render_info.screenshot_rect);
            }
        }
        if resp.drag_stopped() {
            self.state = MoveResizeState::None;
        }
    }

    /// handle move event
    pub fn handle_move(
        &mut self,
        ui: &mut Ui,
        resp: &Response,
        render_info: &RenderInfo,
        rect: &mut Rect,
    ) {
        if resp.dragged()
            && let Some(current_pos) = resp.interact_pointer_pos()
        {
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            if resp.drag_started() {
                let render_range = from_ratio_rect(rect, &render_info.screenshot_rect);
                self.state = MoveResizeState::Move {
                    offset: current_pos - render_range.min,
                    size: render_range.size(),
                };
            }
            if let MoveResizeState::Move { offset, size } = self.state {
                let mut allowed_range = render_info.screenshot_rect;
                allowed_range.max -= size;
                let new_min = allowed_range.clamp(current_pos - offset);
                let new_max = new_min + size;
                let new_range = Rect::from_two_pos(new_min, new_max);
                *rect = to_ratio_rect(&new_range, &render_info.screenshot_rect);
            }
        }
        if resp.drag_stopped() {
            self.state = MoveResizeState::None;
        }
    }

    /// handle move event
    pub fn handle_resize_side(
        &mut self,
        resp: &Response,
        render_info: &RenderInfo,
        rect: &mut Rect,
        init_info: impl FnOnce() -> ResizeSideInfo,
    ) {
        if resp.dragged()
            && let Some(current_pos) = resp.interact_pointer_pos()
        {
            if resp.drag_started() {
                self.state = MoveResizeState::ResizeSide(init_info());
            }
            if let MoveResizeState::ResizeSide(ResizeSideInfo {
                fixed,
                length,
                is_x,
            }) = self.state
            {
                let other_pos = if is_x {
                    Pos2 {
                        x: current_pos.x,
                        y: fixed.y + length,
                    }
                } else {
                    Pos2 {
                        x: fixed.x + length,
                        y: current_pos.y,
                    }
                };
                let new_range = Rect::from_two_pos(fixed, other_pos);
                *rect = to_ratio_rect(&new_range, &render_info.screenshot_rect);
            }
        }
        if resp.drag_stopped() {
            self.state = MoveResizeState::None;
        }
    }
}

pub fn add_control_point(ui: &mut Ui, pos: Pos2, icon: CursorIcon) -> Response {
    const FILL_COLOR: Color32 = Color32::from_gray(0xee);
    const RADIUS: f32 = 6f32;
    const INTERACT_RANGE: f32 = RADIUS + 5f32;

    const STROKE: Stroke = Stroke {
        width: 1.0f32,
        color: Color32::from_gray(0xe),
    };
    ui.painter().rect(
        Rect::from_center_size(pos, Vec2::splat(RADIUS)),
        0f32,
        FILL_COLOR,
        STROKE,
        StrokeKind::Middle,
    );
    ui.allocate_rect(
        Rect::from_center_size(pos, Vec2::splat(INTERACT_RANGE)),
        Sense::drag(),
    )
    .on_hover_cursor(icon)
}
