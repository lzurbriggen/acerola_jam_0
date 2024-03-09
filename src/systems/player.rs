use macroquad::prelude::*;

use crate::{entity::entities::Ecs, game_data::GameData, input_manager::Action};

pub fn update_player(data: &mut GameData, ecs: &mut Ecs) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.velocities.contains_key(e)
    });

    for player in &players {
        let player_data = ecs.components.player_data.get_mut(player).unwrap();
        let velocity = ecs.components.velocities.get_mut(player).unwrap();

        player_data.aberration_increase_timer.update();
        if !data.current_room.completed
            && !data.current_room.aberration_completed
            && player_data.aberration_increase_timer.just_completed()
        {
            player_data.aberration += 0.01;
            player_data.aberration = player_data.aberration.clamp(0., 1.);
        }
        data.graphics
            .aberration_meter_material
            .set_uniform("intensity", player_data.aberration * 2.2);

        let mut dir = Vec2::ZERO;
        if data.input.is_currently_pressed(Action::Left) {
            dir += vec2(-1., 0.);
        }
        if data.input.is_currently_pressed(Action::Up) {
            dir += vec2(0., -1.);
        }
        if data.input.is_currently_pressed(Action::Right) {
            dir += vec2(1., 0.);
        }
        if data.input.is_currently_pressed(Action::Down) {
            dir += vec2(0., 1.);
        }
        if let Some(gamepad) = data.input.gamepads.get_last_used() {
            let input = vec2(gamepad.left_stick_x(), -gamepad.left_stick_y());
            if input.length_squared() > 0. {
                dir = input;
            }
        }

        if dir.length_squared() > 0. {
            *velocity = player_data.move_speed * dir.normalize();
        } else {
            *velocity = Vec2::ZERO;
        };
    }
}
