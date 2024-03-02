use macroquad::math::vec2;

use crate::game_data::GameData;

pub fn draw_hp(data: &GameData, hp: u8, max_hp: u8) {
    let start_pos = vec2(16., 0.);

    for i in 0..max_hp {
        let heart_index = if i < hp { 0 } else { 2 };
        data.sprites.hud_heart.draw(
            start_pos.x + (vec2(i as f32 * 16., start_pos.y)),
            heart_index,
        )
    }
}
