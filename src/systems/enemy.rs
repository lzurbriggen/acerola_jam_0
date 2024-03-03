use macroquad::prelude::*;

use crate::{
    entity::entities::{self, Entities},
    game_data::GameData,
    map::map::Map,
    physics::collision::resolve_map_collision,
};

pub fn update_enemies(data: &GameData, map: &Map, entities: &mut Entities) {
    for enemy in &mut entities.enemies {
        match enemy {
            entities::Enemy::Hopper(hopper) => {
                if hopper.timer.just_completed() {
                    hopper.jumping = !hopper.jumping;

                    if hopper.jumping {
                        hopper.timer.time = 0.8;
                        hopper.timer.reset();
                    } else {
                        hopper.timer.time = rand::gen_range(0.5, 1.5);
                        hopper.timer.reset();
                    }
                }
                let velocity = if hopper.jumping {
                    if hopper.velocity.length_squared() >= 0.1 {
                        hopper.velocity.normalize() * hopper.jump_move_speed
                    } else {
                        Vec2::ZERO
                    }
                } else {
                    (entities.player.position - hopper.position).normalize() * hopper.move_speed
                };
                hopper.velocity = velocity;
                let desired_pos = hopper.position + velocity * get_frame_time();
                hopper.position =
                    resolve_map_collision(data, map, desired_pos, hopper.collider_radius);
            }
        }
    }
}
