use std::ops::Index;

use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use crate::settings::{GameSettings, WindowSize};

use super::{button::button, nine_slice::nice_slice, switcher::switcher, ui_data::UIData};

pub fn pause_menu(ui_data: &UIData, settings: &mut GameSettings) -> bool {
    let mut should_quit = false;

    let frame_size = vec2(200., 220.);
    let center = vec2(360. / 2., 240. / 2.);
    nice_slice(
        &ui_data.frame_texture,
        &RectOffset::new(3., 3., 3., 3.),
        &Rect::new(
            360. / 2. - frame_size.x / 2.,
            240. / 2. - frame_size.y / 2.,
            frame_size.x,
            frame_size.y,
        ),
    );

    let text_size = 16;

    let menu_title = "Game paused";
    let text_center = get_text_center(menu_title, Some(&ui_data.font), text_size, 1., 0.);
    draw_text_ex(
        menu_title,
        center.x - text_center.x,
        14. + text_size as f32 - 2. + 1.,
        TextParams {
            font_size: text_size,
            font: Some(&ui_data.font),
            color: ui_data.text_shadow_color,
            ..Default::default()
        },
    );
    draw_text_ex(
        menu_title,
        center.x - text_center.x,
        14. + text_size as f32 - 2.,
        TextParams {
            font_size: text_size,
            font: Some(&ui_data.font),
            color: ui_data.text_color,
            ..Default::default()
        },
    );

    let switcher_width = 140.;

    // Window size
    let window_size_list = WindowSize::list();
    let current_index = window_size_list
        .iter()
        .position(|s| s == &settings.window_size)
        .unwrap();
    let window_size_text = settings.window_size.text();
    match switcher(
        ui_data,
        settings,
        &Rect::new(center.x - switcher_width / 2., 40., switcher_width, 0.),
        "Window Size",
        &window_size_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let index = if current_index as i8 - 1 < 0 {
                window_size_list.len() - 1
            } else {
                current_index - 1
            };
            settings.set_window_size(window_size_list[index]);
        }
        super::switcher::SwitcherAction::Right => {
            let index = if current_index + 1 > window_size_list.len() - 1 {
                0
            } else {
                current_index + 1
            };
            settings.set_window_size(window_size_list[index]);
        }
        _ => {}
    }

    // Music Volume
    let music_volume_text = format!("{:.0}", settings.music_volume_lin * 100.);
    match switcher(
        ui_data,
        settings,
        &Rect::new(center.x - switcher_width / 2., 80., switcher_width, 0.),
        "Music Volume",
        &music_volume_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let new_vol = (settings.music_volume_lin - 0.05).clamp(0., 1.);
            settings.set_music_volume_lin(new_vol);
        }
        super::switcher::SwitcherAction::Right => {
            let new_vol = (settings.music_volume_lin + 0.05).clamp(0., 1.);
            settings.set_music_volume_lin(new_vol);
        }
        _ => {}
    }

    // SFX Volume
    let sfx_volume_text = format!("{:.0}", settings.sfx_volume_lin * 100.);
    match switcher(
        ui_data,
        settings,
        &Rect::new(center.x - switcher_width / 2., 120., switcher_width, 0.),
        "SFX Volume",
        &sfx_volume_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let new_vol = (settings.sfx_volume_lin - 0.05).clamp(0., 1.);
            settings.set_sfx_volume_lin(new_vol);
        }
        super::switcher::SwitcherAction::Right => {
            let new_vol = (settings.sfx_volume_lin + 0.05).clamp(0., 1.);
            settings.set_sfx_volume_lin(new_vol);
        }
        _ => {}
    }

    let button_width = 100.;
    #[cfg(not(target_arch = "wasm32"))]
    if button(
        ui_data,
        &Rect::new(center.x - switcher_width / 2., 190., button_width, 20.),
        "Leave Game",
        None,
        Vec2::ZERO,
    ) {
        should_quit = true;
    }

    should_quit
}
