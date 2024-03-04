use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    entity::{entities::Ecs, entity_id::Entity, tags::DamageSource},
    game_data::GameData,
    input_manager::Action,
    physics::collision::{check_collision_circles, Collision},
};

pub fn update_player(
    data: &mut GameData,
    collisions: &HashMap<(Entity, Entity), Collision>,
    ecs: &mut Ecs,
) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.velocities.contains_key(e)
            && comps.health.contains_key(e)
    });

    let colliders = ecs.check_components(|e, comps| comps.colliders.contains_key(e));

    for player in &players {
        let player_data = ecs.components.player_data.get_mut(player).unwrap();
        let velocity = ecs.components.velocities.get_mut(player).unwrap();
        let health = ecs.components.health.get_mut(player).unwrap();

        player_data.invulnerable_timer.update();

        if player_data.invulnerable_timer.completed() {
            'damage: for ((source, target), _collision) in collisions.iter() {
                if target != player {
                    continue;
                }
                if let Some(damage_on_coll) = ecs.components.damage_on_collision.get(source) {
                    if damage_on_coll.source != DamageSource::Enemy {
                        continue;
                    }
                    health.hp -= damage_on_coll.damage;
                    player_data.invulnerable_timer.reset();
                    player_data.invulnerable_timer.set_paused(false);
                    break 'damage;
                }
            }
        }

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

    for player in &players {
        let position = ecs.components.positions.get(player).unwrap();
        let player_collider = ecs.components.colliders.get(player).unwrap();
        for other_collider in &colliders {
            if other_collider != player {
                let other_collider_pos = ecs.components.positions.get(&other_collider).unwrap();
                let other_collider = ecs.components.colliders.get(&other_collider).unwrap();

                if check_collision_circles(
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
