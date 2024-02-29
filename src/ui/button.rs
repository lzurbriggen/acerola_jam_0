use std::ops::Add;

use macroquad::{miniquad::window::set_mouse_cursor, prelude::*};

use super::{in_rect::is_in_rect, nine_slice::nice_slice, ui_data::UIData};

pub fn button(
    ui_data: &UIData,
    rect: &Rect,
    text: &str,
    font: Option<&Font>,
    text_offset: Vec2,
) -> bool {
    let mut hover = false;
    let mut pressed = false;
    let mut clicked = false;
    if is_in_rect(mouse_position(), rect) {
        hover = true;
        set_mouse_cursor(miniquad::CursorIcon::Pointer);
        if is_mouse_button_released(MouseButton::Left) {
            clicked = true;
        }
        if is_mouse_button_down(MouseButton::Left) {
            pressed = true;
        }
    }
    let texture = if pressed {
        &ui_data.button_texture_pressed
    } else if hover {
        &ui_data.button_texture_hover
    } else {
        &ui_data.button_texture
    };

    nice_slice(texture, &RectOffset::new(3., 3., 3., 3.), rect);

    let text_offset = if pressed { vec2(0., 0.) } else { vec2(0., -1.) }.add(text_offset);
    let font_size = 16;
    let font = font.unwrap_or(&ui_data.font);
    let center = get_text_center(text, Some(&ui_data.font), font_size, 1., 0.);
    draw_text_ex(
        text,
        rect.x + rect.w / 2. - center.x + text_offset.x,
        rect.y + rect.h / 2. - center.y + text_offset.y,
        TextParams {
            font: Some(&font),
            font_size,
            ..Default::default()
        },
    );

    clicked
}
