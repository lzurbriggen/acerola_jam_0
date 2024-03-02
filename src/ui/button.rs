use std::ops::Add;

use macroquad::{miniquad::window::set_mouse_cursor, prelude::*};

use crate::{game_data::GameData, input_manager::Action};

use super::{in_rect::is_in_rect, nine_slice::nice_slice};

pub fn button(
    data: &GameData,
    rect: &Rect,
    focused: bool,
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
    if focused {
        if data.input.is_currently_pressed(Action::Confirm) {
            pressed = true;
        }
        if data.input.is_just_released(Action::Confirm) {
            clicked = true;
        }
    }

    let texture = if pressed {
        &data.ui.button_texture_pressed
    } else if hover || focused {
        &data.ui.button_texture_hover
    } else {
        &data.ui.button_texture
    };

    nice_slice(texture, &RectOffset::new(3., 3., 3., 3.), rect);

    let text_offset = if pressed { vec2(0., 0.) } else { vec2(0., -1.) }.add(text_offset);
    let font_size = 16;
    let font = font.unwrap_or(&data.ui.font);
    let center = get_text_center(text, Some(&data.ui.font), font_size, 1., 0.);
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
