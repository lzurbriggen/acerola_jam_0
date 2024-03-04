use macroquad::math::vec2;

use crate::{
    entity::{entities::Components, entity_id::Entity},
    game_data::GameData,
};

pub fn draw_hp(data: &GameData, entities: &Vec<Entity>, comps: &Components) {
    let players = entities
        .iter()
        .filter(|e| comps.player_data.contains_key(e))
        .collect::<Vec<&Entity>>();

    let start_pos = vec2(16., 0.);

    for player_e in players {
        let player = comps.player_data.get(player_e).unwrap();

        for i in 0..player.max_hp {
            let heart_index = if i < player.hp { 0 } else { 2 };
            data.sprites.hud_heart.draw(
                start_pos.x + (vec2(i as f32 * 16., start_pos.y)),
                heart_index,
            )
        }
    }
}
