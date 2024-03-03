use macroquad::prelude::*;

use crate::{
    entity::entities::Entities, game_data::GameData, input_manager::Action, map::map::Map,
    physics::collision::resolve_map_collision,
};

pub fn update_player(data: &mut GameData, map: &Map, entities: &mut Entities) {
    let player = &mut entities.player;
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

    let mut desired_pos = if dir.length_squared() > 0. {
        dir = dir.normalize();
        player.position + player.move_speed * dir * get_frame_time()
    } else {
        player.position
    };

    player.position = resolve_map_collision(data, map, desired_pos, player.collider_radius);
}
