use macroquad::{
    color::{Color, WHITE},
    math::{vec2, Vec2},
    texture::{draw_texture, draw_texture_ex, Texture2D},
};

use crate::{entity::entities::Ecs, game_data::GameData, sprite::indexed_sprite::IndexedSprite};

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

    pub fn draw(&self, data: &GameData, ecs: &Ecs) {
        let mirituhgs = ecs.check_components(|e, comps| {
            comps.mirituhg.contains_key(e) && comps.health.contains_key(e)
        });

        let start_pos = vec2(16., 0.);
        for mirituhg_e in mirituhgs {
            let mirituhg = ecs.components.mirituhg.get(&mirituhg_e).unwrap();
            let health = ecs.components.health.get(&mirituhg_e).unwrap();

            draw_texture(&self.hud_texture, 0., 0., WHITE);
        }
    }
}
