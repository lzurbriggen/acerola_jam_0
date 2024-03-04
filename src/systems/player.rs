use macroquad::prelude::*;

use crate::{
    entity::{entities::Components, entity_id::Entity},
    game_data::GameData,
    input_manager::Action,
    map::map::Map,
    physics::collision::{check_collision_circles, resolve_map_collision},
};

pub fn update_player(
    data: &mut GameData,
    map: &Map,
    entities: &Vec<Entity>,
    components: &mut Components,
) {
    let players = entities
        .iter()
        .filter(|e| {
            components.player_data.contains_key(e)
                && components.positions.contains_key(e)
                && components.colliders.contains_key(e)
                && components.velocities.contains_key(e)
        })
        .collect::<Vec<&Entity>>();

    let colliders = entities
        .iter()
        .filter(|e| components.colliders.contains_key(e))
        .collect::<Vec<&Entity>>();

    for player in &players {
        let player_data = components.player_data.get_mut(player).unwrap();
        let velocity = components.velocities.get_mut(player).unwrap();

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

        // let desired_pos = if dir.length_squared() > 0. {
        //     dir = dir.normalize();
        //     *position + player_data.move_speed * dir * get_frame_time()
        // } else {
        //     *position
        // };

        // *position = resolve_map_collision(data, map, desired_pos, player_collider.radius);
    }

    for player in &players {
        let position = components.positions.get(player).unwrap();
        let player_collider = components.colliders.get(player).unwrap();
        for other_collider in &colliders {
            if other_collider != player {
                let other_collider_pos = components.positions.get(&other_collider).unwrap();
                let other_collider = components.colliders.get(&other_collider).unwrap();

                if check_collision_circles(
                    data,
                    *position,
                    player_collider.radius,
                    *other_collider_pos,
                    other_collider.radius,
                )
                .is_some()
                {
                    // println!("{:?}", "other_coll");
                }
            }
        }
    }
}
