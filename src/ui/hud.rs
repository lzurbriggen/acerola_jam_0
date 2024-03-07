use macroquad::{
    color::Color,
    math::vec2,
    shapes::{draw_rectangle_ex, DrawRectangleParams},
};

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

pub fn draw_aberration_meter(data: &GameData, ecs: &Ecs) {
    let players = ecs.check_components(|e, comps| comps.player_data.contains_key(e));

    let pos = vec2(310., 78.);
    for player_e in players {
        let player = ecs.components.player_data.get(&player_e).unwrap();

        data.sprites.aberration_meter.draw(pos, 0);
        let h = player.aberration * 65.;
        draw_rectangle_ex(
            pos.x + 9.,
            pos.y + 16. + 65. - h,
            30.,
            h,
            DrawRectangleParams {
                color: Color::from_hex(0x793a80),
                ..Default::default()
            },
        );
        data.sprites
            .aberration_meter
            .draw(pos + vec2(0., (1. - player.aberration) * 65.), 2);

        data.sprites.aberration_meter.draw(pos, 1);
    }
}
