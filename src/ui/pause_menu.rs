use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
    ui::hash,
};

use crate::{game_data::GameData, input_manager::Action, settings::WindowSize};

use super::{button::button, nine_slice::nice_slice, switcher::switcher};

pub fn pause_menu(data: &mut GameData) -> bool {
    let mut should_quit = false;

    let window_size_id = hash!();
    let music_volume_id = hash!();
    let sfx_volume_id = hash!();
    let show_fps_id = hash!();
    let leave_game_id = hash!();

    let ids = vec![
        window_size_id,
        music_volume_id,
        sfx_volume_id,
        show_fps_id,
        leave_game_id,
    ];

    if data.ui.focus.is_none() || !ids.contains(&data.ui.focus.unwrap()) {
        data.ui.focus = Some(ids[0]);
    }
    let focus = data.ui.focus.unwrap();

    let current_index = ids.iter().position(|s| s == &focus).unwrap();
    if data.input.is_just_pressed(Action::Up) {
        let index = if current_index as i8 - 1 < 0 {
            ids.len() - 1
        } else {
            current_index - 1
        };
        data.ui.focus = Some(ids[index]);
        audio::play_sound(
            &data.audio.ui_switch,
            PlaySoundParams {
                volume: data.settings.sfx_volume,
                ..Default::default()
            },
        );
    } else if data.input.is_just_pressed(Action::Down) {
        let index = if current_index + 1 > ids.len() - 1 {
            0
        } else {
            current_index + 1
        };
        data.ui.focus = Some(ids[index]);
        audio::play_sound(
            &data.audio.ui_switch,
            PlaySoundParams {
                volume: data.settings.sfx_volume,
                ..Default::default()
            },
        );
    }

    let frame_size = vec2(200., 220.);
    let center = vec2(360. / 2., 240. / 2.);
    nice_slice(
        &data.ui.frame_texture,
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
    let text_center = get_text_center(menu_title, Some(&data.ui.font), text_size, 1., 0.);
    draw_text_ex(
        menu_title,
        center.x - text_center.x,
        14. + text_size as f32 - 2. + 1.,
        TextParams {
            font_size: text_size,
            font: Some(&data.ui.font),
            color: data.ui.text_shadow_color,
            ..Default::default()
        },
    );
    draw_text_ex(
        menu_title,
        center.x - text_center.x,
        14. + text_size as f32 - 2.,
        TextParams {
            font_size: text_size,
            font: Some(&data.ui.font),
            color: data.ui.text_color,
            ..Default::default()
        },
    );

    let switcher_width = 140.;

    // Window size
    let window_size_list = WindowSize::list();
    let current_index = window_size_list
        .iter()
        .position(|s| s == &data.settings.window_size)
        .unwrap();
    let window_size_text = data.settings.window_size.text();
    match switcher(
        data,
        &data.settings,
        &Rect::new(center.x - switcher_width / 2., 40., switcher_width, 0.),
        "Window Size",
        data.ui.focus.is_some() && window_size_id == data.ui.focus.unwrap(),
        &window_size_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let index = if current_index as i8 - 1 < 0 {
                window_size_list.len() - 1
            } else {
                current_index - 1
            };
            data.settings.set_window_size(window_size_list[index]);
        }
        super::switcher::SwitcherAction::Right => {
            let index = if current_index + 1 > window_size_list.len() - 1 {
                0
            } else {
                current_index + 1
            };
            data.settings.set_window_size(window_size_list[index]);
        }
        _ => {}
    }

    // Music Volume
    let music_volume_text = format!("{:.0}", data.settings.music_volume_lin * 100.);
    match switcher(
        data,
        &data.settings,
        &Rect::new(center.x - switcher_width / 2., 80., switcher_width, 0.),
        "Music Volume",
        data.ui.focus.is_some() && music_volume_id == data.ui.focus.unwrap(),
        &music_volume_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let new_vol = (data.settings.music_volume_lin - 0.05).clamp(0., 1.);
            data.settings.set_music_volume_lin(new_vol);
        }
        super::switcher::SwitcherAction::Right => {
            let new_vol = (data.settings.music_volume_lin + 0.05).clamp(0., 1.);
            data.settings.set_music_volume_lin(new_vol);
        }
        _ => {}
    }

    // SFX Volume
    let sfx_volume_text = format!("{:.0}", data.settings.sfx_volume_lin * 100.);
    match switcher(
        data,
        &data.settings,
        &Rect::new(center.x - switcher_width / 2., 120., switcher_width, 0.),
        "SFX Volume",
        data.ui.focus.is_some() && sfx_volume_id == data.ui.focus.unwrap(),
        &sfx_volume_text,
    ) {
        super::switcher::SwitcherAction::Left => {
            let new_vol = (data.settings.sfx_volume_lin - 0.05).clamp(0., 1.);
            data.settings.set_sfx_volume_lin(new_vol);
        }
        super::switcher::SwitcherAction::Right => {
            let new_vol = (data.settings.sfx_volume_lin + 0.05).clamp(0., 1.);
            data.settings.set_sfx_volume_lin(new_vol);
        }
        _ => {}
    }

    let button_width = 70.;
    if button(
        data,
        &Rect::new(center.x - button_width / 2., 165., button_width, 20.),
        data.ui.focus.is_some() && show_fps_id == data.ui.focus.unwrap(),
        if data.show_fps {
            "Hide FPS"
        } else {
            "Show FPS"
        },
        None,
        Vec2::ZERO,
    ) {
        data.show_fps = !data.show_fps;
        audio::play_sound(
            &data.audio.confirm,
            PlaySoundParams {
                volume: data.settings.sfx_volume,
                ..Default::default()
            },
        );
    }

    let button_width = 100.;
    #[cfg(not(target_arch = "wasm32"))]
    if button(
        data,
        &Rect::new(center.x - button_width / 2., 200., button_width, 20.),
        data.ui.focus.is_some() && leave_game_id == data.ui.focus.unwrap(),
        "Leave Game",
        None,
        Vec2::ZERO,
    ) {
        should_quit = true;
        audio::play_sound(
            &data.audio.confirm,
            PlaySoundParams {
                volume: data.settings.sfx_volume,
                ..Default::default()
            },
        );
    }

    should_quit
}
