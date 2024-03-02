use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use crate::{game_data::GameData, input_manager::Action, settings::GameSettings};

use super::{button::button, nine_slice};

pub enum SwitcherAction {
    Left,
    Right,
    None,
}

pub fn switcher(
    data: &GameData,
    settings: &GameSettings,
    rect: &Rect,
    label_text: &str,
    focused: bool,
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
            font: Some(&data.ui.font),
            color: data.ui.text_color,
            ..Default::default()
        },
    );

    let y = rect.y + 18.;

    if focused {
        let focus_bg_offset = 2.;
        nine_slice::nice_slice(
            &data.ui.focus_background_texture,
            &RectOffset::new(3., 3., 3., 3.),
            &Rect::new(
                rect.x - focus_bg_offset,
                y - focus_bg_offset,
                rect.w + 2. * focus_bg_offset,
                20. + 2. * focus_bg_offset,
            ),
        )
    }

    let mut input_left = focused && data.input.is_just_released(Action::Left);
    let mut input_right = focused && data.input.is_just_released(Action::Right);

    if button(
        &data,
        &Rect::new(rect.x, y, button_width, 20.),
        false,
        "½",
        Some(&data.ui.icon_font),
        vec2(-3., -1.),
    ) || input_left
    {
        action = SwitcherAction::Left;
        audio::play_sound(
            &data.ui.button_click_sfx,
            PlaySoundParams {
                volume: settings.sfx_volume,
                ..Default::default()
            },
        );
    }
    let text_center = get_text_center(text, Some(&data.ui.font), text_size, 1., 0.);
    draw_text_ex(
        text,
        rect.x + rect.w / 2. - text_center.x,
        y + text_size as f32 - 2.,
        TextParams {
            font_size: text_size,
            font: Some(&data.ui.font),
            ..Default::default()
        },
    );
    if button(
        &data,
        &Rect::new(rect.x + rect.w - button_width, y, button_width, 20.),
        false,
        "¾",
        Some(&data.ui.icon_font),
        vec2(-2., -1.),
    ) || input_right
    {
        action = SwitcherAction::Right;
        audio::play_sound(
            &data.ui.button_click_sfx,
            PlaySoundParams {
                volume: settings.sfx_volume,
                ..Default::default()
            },
        );
    }

    action
}
