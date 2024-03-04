use macroquad::prelude::*;

use crate::entity::entities::Ecs;

pub fn update_enemies(ecs: &mut Ecs) {
    let hoppers = ecs.check_components(|e, comps| {
        comps.hoppers.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.velocities.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.animated_sprites.contains_key(e)
    });

    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.positions.contains_key(e)
    });

    for hopper_e in &hoppers {
        let player_pos = {
            let mut pos = Vec2::ZERO;
            for player_e in &players {
                pos = *ecs.components.positions.get(player_e).unwrap();
                break;
            }
            pos
        };
        let hopper = ecs.components.hoppers.get_mut(hopper_e).unwrap();
        let position = ecs.components.positions.get_mut(hopper_e).unwrap();
        let velocity = ecs.components.velocities.get_mut(hopper_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(hopper_e).unwrap();

        hopper.timer.update();

        if hopper.timer.just_completed() {
            hopper.jumping = !hopper.jumping;

            if hopper.jumping {
                sprite.set_animation("jump");
                hopper.timer.time = 0.96;
                hopper.timer.reset();
            } else {
                sprite.set_animation("move");
                hopper.timer.time = rand::gen_range(0.5, 1.5);
                hopper.timer.reset();
            }
        }
        *velocity = if hopper.jumping {
            if velocity.length_squared() > 0. {
                velocity.normalize() * hopper.jump_move_speed
            } else {
                Vec2::ZERO
            }
        } else {
            (player_pos - *position).normalize() * hopper.move_speed
        };
    }
}
