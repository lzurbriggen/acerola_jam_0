use macroquad::{prelude::*, ui::hash};

use crate::{game_data::GameData, input_manager::Action, settings::WindowSize, timer::Timer};

use super::{button::button, nine_slice::nice_slice, switcher::switcher};

pub struct UpgradeScreen {
    upgrades: Vec<u64>,
    // texture: Texture2D,
    // start_timer: Timer,
    // timer: Timer,
    // text_timer: Timer,
    // show_button_timer: Timer,
}

impl UpgradeScreen {
    pub fn new() -> Self {
        Self {
            upgrades: vec![hash!(), hash!(), hash!()],
        }
    }

    pub fn draw(&self, data: &mut GameData) -> bool {
        let mut should_quit = false;

        let ids = &self.upgrades;
        if data.ui.focus.is_none() || !ids.contains(&data.ui.focus.unwrap()) {
            data.ui.focus = Some(ids[0]);
        }
        let focus = data.ui.focus.unwrap();

        let current_index = ids.iter().position(|s| s == &focus).unwrap();
        if data.input.is_just_pressed(Action::Left) {
            let index = if current_index as i8 - 1 < 0 {
                ids.len() - 1
            } else {
                current_index - 1
            };
            data.ui.focus = Some(ids[index]);
        } else if data.input.is_just_pressed(Action::Right) {
            let index = if current_index + 1 > ids.len() - 1 {
                0
            } else {
                current_index + 1
            };
            data.ui.focus = Some(ids[index]);
        }

        let container_size = vec2(360. * 0.8, 240. * 0.8);
        let spacing = 10.;
        let container_pos = vec2(
            (360. - container_size.x) / 2.,
            (240. - container_size.y) / 2.,
        );

        let len_f32 = self.upgrades.len() as f32;
        let frame_size = vec2(
            (container_size.x - (len_f32 - 1.) * spacing) / len_f32,
            container_size.y,
        );
        for (i, _upgrade) in self.upgrades.iter().enumerate() {
            let frame_rect = Rect::new(
                (container_pos.x + i as f32 * (frame_size.x + spacing)).round(),
                container_pos.y,
                frame_size.x.round(),
                frame_size.y,
            );
            if focus == ids[i] {
                nice_slice(
                    &data.ui.frame_texture_pretty,
                    &RectOffset::new(8., 8., 8., 8.),
                    &frame_rect,
                );
            } else {
                nice_slice(
                    &data.ui.frame_texture,
                    &RectOffset::new(3., 3., 3., 3.),
                    &frame_rect,
                );
            }
        }

        should_quit
    }
}
