use std::f32::consts::{PI, TAU};

use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
    rand::gen_range,
};

use crate::{
    entity::{
        entities::Ecs, events::DamageEvent, hopper::spawn_hopper, mirituhg::MiritughState,
        projectile::spawn_bullet, tags::EntityType,
    },
    game_data::GameData,
    rand_utils::rand_dir,
};

use super::collision::ColliderType;

pub fn update_enemies(data: &mut GameData, ecs: &mut Ecs, damage_events: &mut Vec<DamageEvent>) {
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
        let position = ecs.components.positions.get(hopper_e).unwrap();
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
        let vel = if hopper.jumping {
            if velocity.length_squared() > 0. {
                velocity.normalize() * hopper.jump_move_speed
            } else {
                Vec2::ZERO
            }
        } else {
            (player_pos - *position).normalize() * hopper.move_speed
        };

        if !vel.is_nan() {
            *velocity = vel;
        }
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
        let position = ecs.components.positions.get(spitter_e).unwrap();
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
        spawn_bullet(
            data,
            ecs,
            pos,
            EntityType::Player,
            1.,
            vel,
            ColliderType::Projectile,
        );
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
        let position = ecs.components.positions.get(stomper_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(stomper_e).unwrap();
        let velocity = ecs.components.velocities.get_mut(stomper_e).unwrap();

        stomper.damage_timer.update();
        stomper.jump_timer.update();

        let dist_to_player = (player_pos - *position).length();
        // TODO: check range to player to start
        if dist_to_player < 36. && !stomper.jumping && stomper.jump_timer.completed() {
            sprite.set_animation("jump");
            stomper.damage_timer.reset();
            stomper.jumping = true
        }
        if stomper.jumping {
            let vel = (player_pos - *position).normalize() * stomper.jump_move_speed;
            if !vel.is_nan() {
                *velocity = vel;
            }
            if stomper.damage_timer.just_completed() && dist_to_player < 26. {
                for player_e in &players {
                    damage_events.push(DamageEvent {
                        source: *stomper_e,
                        target: *player_e,
                        damage: 1.,
                    });
                }
            }

            if sprite.current_animation().1.completed {
                stomper.jumping = false;
                sprite.set_animation("walk");
                stomper.jump_timer.reset();
            }
        } else {
            let vel = (player_pos - *position).normalize() * stomper.move_speed;
            if !vel.is_nan() {
                *velocity = vel;
            }
        }
    }

    let mirituhgs = ecs.check_components(|e, comps| {
        comps.mirituhg.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.velocities.contains_key(e)
            && comps.colliders.contains_key(e)
            && comps.animated_sprites.contains_key(e)
    });

    let mut hopper_spawns = vec![];
    let mut bullets = vec![];

    for mirituhg_e in &mirituhgs {
        let mirituhg = ecs.components.mirituhg.get_mut(mirituhg_e).unwrap();
        let position = ecs.components.positions.get(mirituhg_e).unwrap();
        let sprite = ecs.components.animated_sprites.get_mut(mirituhg_e).unwrap();
        let velocity = ecs.components.velocities.get_mut(mirituhg_e).unwrap();

        mirituhg.update();

        let dist = mirituhg.target_pos - *position;
        if mirituhg.state == MiritughState::Idle {
            if mirituhg.next_move_timer.just_completed() || dist.length_squared() < 4. {
                mirituhg.target_pos = vec2(180., 120.) + rand_dir() * gen_range(50., 72.);
            }
        }

        if mirituhg.state == MiritughState::Spawn {
            mirituhg.target_pos = vec2(180., 120.);
            if !mirituhg.spawning && dist.length_squared() < 4. {
                mirituhg.spawning = true;
                sprite.set_animation("spawn");
                mirituhg.spawn_1_timer.reset();
                mirituhg.spawn_2_timer.reset();
                mirituhg.spawn_3_timer.reset();
            }
            if mirituhg.spawn_1_timer.just_completed()
                || mirituhg.spawn_2_timer.just_completed()
                || mirituhg.spawn_3_timer.just_completed()
            {
                hopper_spawns.push(());
            }

            if mirituhg.spawning && sprite.current_animation().1.completed {
                mirituhg.spawning = false;
                mirituhg.state = MiritughState::Idle;
                mirituhg.next_attack_timer.reset();
                mirituhg.previously_spawned = true;
                sprite.set_animation("idle");
            }
        }

        if mirituhg.state == MiritughState::ShootTransition {
            mirituhg.target_pos = vec2(180., 120.);
            if dist.length_squared() < 4.
                && sprite.current_animation != "shoot_transition".to_string()
            {
                mirituhg.shoot_timer.reset();
                sprite.set_animation("shoot_transition");
            }
            if sprite.current_animation == "shoot_transition".to_string()
                && sprite.current_animation().1.completed
            {
                mirituhg.state = MiritughState::Shoot;
                mirituhg.next_attack_timer.reset();
                mirituhg.shoot_timer.reset();
            }
        }

        if mirituhg.state == MiritughState::Shoot {
            if mirituhg.next_move_timer.just_completed() || dist.length_squared() < 4. {
                mirituhg.target_pos = vec2(180., 120.) + rand_dir() * gen_range(50., 72.);
            }
            if mirituhg.shoot_timer.just_completed() {
                mirituhg.shoot_timer.reset();
                for i in 0..8 {
                    bullets.push((
                        *position,
                        Vec2::from_angle(mirituhg.shoot_rotation + TAU / 8. * i as f32) * 50.,
                    ));
                }
            }
        }

        if mirituhg.next_attack_timer.just_completed() {
            match mirituhg.state {
                MiritughState::Idle => {
                    // TODO: check prev spawn
                    if mirituhg.previously_spawned {
                        mirituhg.state = MiritughState::ShootTransition;
                    } else {
                        mirituhg.state = MiritughState::Spawn;
                    }
                }
                MiritughState::Spawn => {}
                MiritughState::ShootTransition => {}
                MiritughState::Shoot => {
                    mirituhg.previously_spawned = false;
                    mirituhg.state = MiritughState::Idle;
                    sprite.set_animation("idle");
                }
                MiritughState::Dead => {}
            }
            mirituhg.next_attack_timer.reset();
        }

        if dist.length_squared() > 0. {
            *velocity = (mirituhg.target_pos - *position).normalize() * mirituhg.move_speed;
        }
    }

    let mirituhgs_deaths = ecs.check_components(|e, comps| {
        comps.mirituhg_death.contains_key(e) && comps.animated_sprites.contains_key(e)
    });

    for mirituhg_e in &mirituhgs_deaths {
        let sprite = ecs.components.animated_sprites.get(mirituhg_e).unwrap();
        if sprite.current_animation == "death".to_string() && sprite.current_animation().1.completed
        {
            data.end_game_screen.show();
            data.paused = true;
            data.game_completed = true;
        }
    }

    for (pos, vel) in bullets {
        spawn_bullet(
            data,
            ecs,
            pos,
            EntityType::Player,
            1.,
            vel,
            ColliderType::Projectile,
        );
    }

    for _ in hopper_spawns {
        spawn_hopper(data, vec2(180., 120.), ecs);
        spawn_hopper(data, vec2(180., 120.), ecs);
        spawn_hopper(data, vec2(180., 120.), ecs);
    }
}
