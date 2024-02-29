use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use crate::settings::GameSettings;

use super::{button::button, ui_data::UIData};

pub enum SwitcherAction {
    Left,
    Right,
    None,
}

pub fn switcher(
    ui_data: &UIData,
    settings: &GameSettings,
    rect: &Rect,
    label_text: &str,
    text: &str,
) -> SwitcherAction {
    let text_size = 16;
    let button_width = 20.;

    let mut action = SwitcherAction::None;

    draw_text_ex(
        label_text,
        rect.x,
        rect.y + text_size as f32 - 2.,
        TextParams {
            font_size: text_size,
            font: Some(&ui_data.font),
            color: ui_data.text_color,
            ..Default::default()
        },
    );

    let y = rect.y + 18.;

    if button(
        ui_data,
        &Rect::new(rect.x, y, button_width, 20.),
        "½",
        Some(&ui_data.icon_font),
        vec2(-3., -1.),
    ) {
        action = SwitcherAction::Left;
        audio::play_sound(
            &ui_data.button_click_sfx,
            PlaySoundParams {
                volume: settings.sfx_volume,
                ..Default::default()
            },
        );
    }
    let text_center = get_text_center(text, Some(&ui_data.font), text_size, 1., 0.);
    draw_text_ex(
        text,
        rect.x + rect.w / 2. - text_center.x,
        y + text_size as f32 - 2.,
        TextParams {
            font_size: text_size,
            font: Some(&ui_data.font),
            ..Default::default()
        },
    );
    if button(
        ui_data,
        &Rect::new(rect.x + rect.w - button_width, y, button_width, 20.),
        "¾",
        Some(&ui_data.icon_font),
        vec2(-2., -1.),
    ) {
        action = SwitcherAction::Right;
        audio::play_sound(
            &ui_data.button_click_sfx,
            PlaySoundParams {
                volume: settings.sfx_volume,
                ..Default::default()
            },
        );
    }

    action
}
