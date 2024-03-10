use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use crate::{
    entity::{entities::Ecs, projectile::spawn_bullet, tags::EntityType},
    game_data::GameData,
};

pub fn update_enemies(data: &mut GameData, ecs: &mut Ecs) {
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

    let player_pos = {
        let mut pos = Vec2::ZERO;
        for player_e in &players {
            pos = *ecs.components.positions.get(player_e).unwrap();
            break;
        }
        pos
    };

    for hopper_e in &hoppers {
        let hopper = ecs.components.hoppers.get_mut(hopper_e).unwrap();
        let position = ecs.components.positions.get_mut(hopper_e).unwrap();
        let velocity = ecs.components.velocities.get_mut(hopper_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(hopper_e).unwrap();

        hopper.jump_timer.update();

        if hopper.jump_timer.just_completed() {
            hopper.jumping = !hopper.jumping;

            if hopper.jumping {
                sprite.set_animation("jump");
                hopper.jump_timer.time = 0.96;
                hopper.jump_timer.reset();
            } else {
                sprite.set_animation("move");
                hopper.jump_timer.time = rand::gen_range(0.5, 1.5);
                hopper.jump_timer.reset();
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

    let spitters = ecs.check_components(|e, comps| {
        comps.spitters.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.animated_sprites.contains_key(e)
    });

    let mut bullets = vec![];
    for spitter_e in &spitters {
        let spitter = ecs.components.spitters.get_mut(spitter_e).unwrap();
        let position = ecs.components.positions.get_mut(spitter_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(spitter_e).unwrap();

        spitter.attack_timer.update();
        spitter.spit_timer.update();

        if spitter.attack_timer.just_completed() {
            sprite.set_animation("spit");
            spitter.attack_timer.reset();
            spitter.spit_timer.reset();
        }
        if spitter.spit_timer.just_completed() {
            let bullet_velocity = (player_pos - *position).normalize() * 50.;
            bullets.push((*position + vec2(0., -5.), bullet_velocity));
            audio::play_sound(
                &data.audio.shoot,
                PlaySoundParams {
                    volume: data.settings.sfx_volume * 0.6,
                    ..Default::default()
                },
            );
        }

        if sprite.current_animation == "spit".to_string() && sprite.current_animation().1.completed
        {
            sprite.set_animation("idle");
        }
    }

    for (pos, vel) in bullets {
        spawn_bullet(data, ecs, pos, EntityType::Player, 1., vel);
    }

    let stompers = ecs.check_components(|e, comps| {
        comps.stompers.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.velocities.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.animated_sprites.contains_key(e)
    });

    for stomper_e in &stompers {
        let stomper = ecs.components.stompers.get_mut(stomper_e).unwrap();
        let position = ecs.components.positions.get_mut(stomper_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(stomper_e).unwrap();
        let velocity = ecs.components.velocities.get_mut(stomper_e).unwrap();

        stomper.attack_timer.update();
        stomper.spit_timer.update();

        *velocity = (player_pos - *position).normalize() * stomper.move_speed;
    }
}
