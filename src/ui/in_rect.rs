use macroquad::prelude::*;

pub fn is_in_rect(pos: (f32, f32), rect: &Rect) -> bool {
    let scale_factor = vec2(screen_width() / 360., screen_height() / 240.);
    let pos = (pos.0 / scale_factor.x, pos.1 / scale_factor.y);

    pos.0 >= rect.x && pos.0 <= rect.x + rect.w && pos.1 >= rect.y && pos.1 <= rect.y + rect.h
}
