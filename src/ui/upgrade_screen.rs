use macroquad::{prelude::*, ui::hash};

use crate::{entity::upgrades::Upgrade, game_data::GameData, input_manager::Action};

use super::{button::button, nine_slice::nice_slice};

pub struct UpgradeScreen {
    upgrades: Vec<Upgrade>,
    ids: Vec<u64>,
}

impl UpgradeScreen {
    pub fn new(upgrades: Vec<Upgrade>) -> Self {
        Self {
            upgrades,
            ids: vec![hash!(), hash!(), hash!()],
        }
    }

    pub fn draw(&self, data: &mut GameData) -> Option<Upgrade> {
        let mut chosen_upgrade: Option<Upgrade> = None;

        let ids = &self.ids;
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

        let container_size = vec2(360. * 0.8, 129. + 26.);
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
        for (i, upgrade) in self.upgrades.iter().enumerate() {
            let is_focused = focus == ids[i];
            let frame_rect = Rect::new(
                (container_pos.x + i as f32 * (frame_size.x + spacing)).round(),
                container_pos.y,
                frame_size.x.round(),
                frame_size.y,
            );

            if is_focused {
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

            let banner_texture_name = match upgrade {
                Upgrade::Item(_) => "upgrade_banner_item",
                Upgrade::Weapon(_) | Upgrade::WeaponUpgrade(_) => "upgrade_banner_weapon",
                Upgrade::CommonUpgrade(_) => "upgrade_banner_upgrade",
            };
            let banner_texture = data.graphics.textures.get(banner_texture_name).unwrap();

            let inner_pos = vec2(frame_rect.x + 2., frame_rect.y + 2.);

            let banner_rect = Rect::new(inner_pos.x, inner_pos.y + 8., 85., 15.);
            let mut y = banner_rect.y + banner_rect.h;
            draw_texture_ex(
                banner_texture,
                banner_rect.x,
                banner_rect.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );

            let banner_text = match upgrade {
                Upgrade::Item(_) => "Item",
                Upgrade::Weapon(_) | Upgrade::WeaponUpgrade(_) => "Weapon",
                Upgrade::CommonUpgrade(_) => "Upgrade",
            };

            let center = get_text_center(banner_text, Some(&data.ui.font), 16, 1., 0.);
            draw_text_ex(
                banner_text,
                banner_rect.x + banner_rect.w / 2. - center.x,
                banner_rect.y + banner_rect.h / 2. - center.y - 1.,
                TextParams {
                    font: Some(&data.ui.font),
                    font_size: 16,
                    ..Default::default()
                },
            );

            if let Upgrade::WeaponUpgrade(_) = upgrade {
                let mut banner_rect = banner_rect.clone();
                banner_rect.y += 15.;
                y = banner_rect.y + banner_rect.h;

                let banner_texture = data
                    .graphics
                    .textures
                    .get("upgrade_banner_upgrade")
                    .unwrap();
                draw_texture_ex(
                    banner_texture,
                    banner_rect.x,
                    banner_rect.y,
                    WHITE,
                    DrawTextureParams {
                        ..Default::default()
                    },
                );

                let banner_text = "Upgrade";
                let center = get_text_center(banner_text, Some(&data.ui.font), 16, 1., 0.);
                draw_text_ex(
                    banner_text,
                    banner_rect.x + banner_rect.w / 2. - center.x,
                    banner_rect.y + banner_rect.h / 2. - center.y - 1.,
                    TextParams {
                        font: Some(&data.ui.font),
                        font_size: 16,
                        ..Default::default()
                    },
                );
            }

            let description = upgrade.description();

            // Upgrade texture frame
            let upgrade_frame_texture = data.graphics.textures.get("upgrade_frame_inner").unwrap();
            let inner_frame_size = Rect::new(inner_pos.x + 4., y + 4., 77., 35.);
            nice_slice(
                upgrade_frame_texture,
                &RectOffset::new(3., 3., 5., 3.),
                &inner_frame_size,
            );
            let upgrade_texture = data
                .graphics
                .textures
                .get(description.texture_name.as_str())
                .unwrap();
            draw_texture_ex(
                upgrade_texture,
                inner_frame_size.x + 2.,
                inner_frame_size.y + 2.,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );

            // Upgrade text frame
            let upgrade_frame_texture = data
                .graphics
                .textures
                .get("upgrade_frame_inner_dark")
                .unwrap();
            let mut inner_frame_size = inner_frame_size.clone();
            inner_frame_size.y += inner_frame_size.h + 6.;
            inner_frame_size.h = description.text.len() as f32 * 18.;
            nice_slice(
                upgrade_frame_texture,
                &RectOffset::new(3., 3., 5., 3.),
                &inner_frame_size,
            );

            for (i, text) in description.text.iter().enumerate() {
                let center = get_text_center(text, Some(&data.ui.font), 16, 1., 0.);
                draw_text_ex(
                    text,
                    inner_frame_size.x + inner_frame_size.w / 2. - center.x,
                    inner_frame_size.y + 14. + i as f32 * 16.,
                    TextParams {
                        font: Some(&data.ui.font),
                        font_size: 16,
                        ..Default::default()
                    },
                );
            }

            if button(
                data,
                &Rect::new(
                    inner_frame_size.x,
                    inner_frame_size.bottom() + 7.,
                    inner_frame_size.w,
                    20.,
                ),
                is_focused,
                "Choose",
                None,
                Vec2::ZERO,
            ) {
                chosen_upgrade = Some(upgrade.clone());
            }
        }

        chosen_upgrade
    }
}
