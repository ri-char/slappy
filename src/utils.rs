use eframe::egui::{Pos2, Rect};

#[inline]
pub fn to_ratio_rect(rect: &Rect, base: &Rect) -> Rect {
    Rect {
        min: to_ratio_pos(&rect.min, base),
        max: to_ratio_pos(&rect.max, base),
    }
}

#[inline]
pub fn from_ratio_rect(rect: &Rect, base: &Rect) -> Rect {
    Rect::from_min_max(
        base.lerp_inside(rect.min.to_vec2()),
        base.lerp_inside(rect.max.to_vec2()),
    )
}

#[inline]
pub fn to_ratio_pos(pos: &Pos2, base: &Rect) -> Pos2 {
    Pos2 {
        x: (pos.x - base.min.x) / base.width(),
        y: (pos.y - base.min.y) / base.height(),
    }
}

#[inline]
pub fn from_ratio_pos(pos: &Pos2, base: &Rect) -> Pos2 {
    base.lerp_inside(pos.to_vec2())
}
