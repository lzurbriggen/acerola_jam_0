use macroquad::{
    color::WHITE,
    math::{Rect, RectOffset},
    texture::{draw_texture, Texture2D},
};

use crate::{entity::entities::Ecs, game_data::GameData};

use super::nine_slice;

pub struct HudMirituhg {
    hud_texture: Texture2D,
    health_bar_texture: Texture2D,
}

impl HudMirituhg {
    pub fn new(hud_texture: Texture2D, health_bar_texture: Texture2D) -> Self {
        Self {
            hud_texture,
            health_bar_texture,
        }
    }

    pub fn draw(&self, _data: &GameData, ecs: &Ecs) {
        let mirituhgs = ecs.check_components(|e, comps| {
            comps.mirituhg.contains_key(e) && comps.health.contains_key(e)
        });

        for mirituhg_e in mirituhgs {
            let mirituhg = ecs.components.mirituhg.get(&mirituhg_e).unwrap();
            let health = ecs.components.health.get(&mirituhg_e).unwrap();

            draw_texture(&self.hud_texture, 0., 0., WHITE);

            nine_slice::nice_slice(
                &self.health_bar_texture,
                &RectOffset::new(3., 2., 2., 2.),
                &Rect::new(65., 217., (health.hp / mirituhg.max_hp) * 222., 6.),
            );
        }
    }
}
