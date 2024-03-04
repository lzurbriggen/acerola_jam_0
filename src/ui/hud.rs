use macroquad::math::vec2;

use crate::{entity::entities::Ecs, game_data::GameData};

pub fn draw_hp(data: &GameData, ecs: &Ecs) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.health.contains_key(e)
    });

    let start_pos = vec2(16., 0.);
    for player_e in players {
        let player = ecs.components.player_data.get(&player_e).unwrap();
        let health = ecs.components.health.get(&player_e).unwrap();

        for i in 0..player.max_hp {
            let heart_index = if (i as f32) < health.hp { 0 } else { 2 };
            data.sprites.hud_heart.draw(
                start_pos.x + (vec2(i as f32 * 16., start_pos.y)),
                heart_index,
            )
        }
    }
}
