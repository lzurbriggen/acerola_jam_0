use std::f32::consts::TAU;

use macroquad::{
    audio::{self, PlaySoundParams},
    input::{is_mouse_button_pressed, MouseButton},
    math::{vec2, Vec2},
};

use crate::{
    entity::{
        entities::Ecs,
        projectile::spawn_bullet,
        tags::{DamageOnCollision, EntityType},
    },
    game_data::GameData,
    input_manager::Action,
    items::weapon::Weapon,
};

pub fn update_weapon(ecs: &mut Ecs, data: &mut GameData) {
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.positions.contains_key(e)
    });

    let ball_entities = ecs.check_components(|e, comps| {
        comps.balls.contains_key(e)
            && comps.positions.contains_key(e)
            && comps.velocities.contains_key(e)
    });

    let mut bullet_data = Vec::<(f32, Vec2, Vec2, Option<usize>)>::new();

    let mut player_pos = Vec2::ZERO;
    for player_e in &players {
        let player_position = ecs.components.positions.get_mut(&player_e).unwrap();
        let player_data = ecs.components.player_data.get_mut(&player_e).unwrap();
        player_pos = *player_position;
        match &mut data.weapon {
            Weapon::Launcher(ref mut launcher) => {
                launcher.shoot_timer.update();

                let launcher_data = launcher.get_upgraded_data();

                if launcher.shoot_timer.completed() {
                    launcher.shoot_timer.reset();

                    let dir = data.input.get_aim_dir(&data.camera, player_pos);

                    bullet_data.push((
                        launcher_data.damage,
                        *player_position + dir * 3.,
                        dir * 160.,
                        None,
                    ));
                    // bullet_data.push((
                    //     shooter.damage,
                    //     *player_position + vec2(-3., 0.),
                    //     vec2(-160., 0.),
                    //     None,
                    // ));
                    // bullet_data.push((
                    //     shooter.damage,
                    //     *player_position + vec2(0., 3.),
                    //     vec2(0., 160.),
                    //     None,
                    // ));
                    // bullet_data.push((
                    //     shooter.damage,
                    //     *player_position + vec2(0., -3.),
                    //     vec2(0., -160.),
                    //     None,
                    // ));
                }

                launcher.shoot_timer.time = launcher_data.timer_duration;
            }
            Weapon::Balls(ref mut balls) => {
                balls.update();

                let balls_data = balls.get_upgraded_data();

                if ball_entities.len() < balls_data.amount && balls.buffered_spawns > 0 {
                    //
                    // let missing_ball_index = ball_entities.iter().find(|e| e.0);
                    let mut ball_ids = (0..balls_data.amount).collect::<Vec<usize>>();
                    for ball_e in &ball_entities {
                        let id = ecs.components.balls.get(ball_e).unwrap();
                        if ball_ids.contains(id) {
                            ball_ids = ball_ids
                                .iter()
                                .filter(|b_id| *b_id != id)
                                .map(|b_id| *b_id)
                                .collect();
                        }
                    }
                    balls.buffered_spawns -= 1;
                    bullet_data.push((
                        balls.base_damage,
                        *player_position,
                        vec2(0., 0.),
                        Some(ball_ids[0]),
                    ));
                    balls.ball_spawn_timer.reset();
                }
            }
            Weapon::Dash(ref mut dash) => {
                dash.update();
                let dash_data = dash.get_upgraded_data();

                if dash.dashing_timer.just_completed() {
                    dash.dashing = false;
                    for shadow_e in &player_data.shadows {
                        let sprite = ecs.components.animated_sprites.get_mut(&shadow_e).unwrap();
                        sprite.visible = false;
                    }
                    ecs.components.damage_on_collision.remove(player_e);
                }
                if dash.shadow_timer.just_completed() && dash.shadow_index < 2 {
                    let shadow_e = player_data.shadows[dash.shadow_index];
                    let sprite = ecs.components.animated_sprites.get_mut(&shadow_e).unwrap();
                    let shadow_pos = ecs.components.positions.get_mut(&shadow_e).unwrap();
                    sprite.visible = true;
                    *shadow_pos = player_pos;
                    dash.shadow_timer.reset();
                    dash.shadow_index += 1;
                }
                if dash.dash_timer.completed() {
                    if data.input.is_just_pressed(Action::Confirm) {
                        dash.dash_timer.reset();
                        dash.dashing_timer.reset();
                        dash.shadow_timer.reset();
                        ecs.components.damage_on_collision.insert(
                            *player_e,
                            DamageOnCollision {
                                source: EntityType::Player,
                                damage: dash_data.damage,
                            },
                        );
                        dash.dashing = true;
                        dash.shadow_index = 0;

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

                        dash.direction = if dir.length_squared() > 0. {
                            dir.normalize()
                        } else {
                            Vec2::X
                        };
                    }
                }
                dash.dash_timer.time = dash_data.dash_timer_duration;
            }
        }
    }

    for ball_e in &ball_entities {
        if let Weapon::Balls(ref mut balls) = data.weapon {
            let balls_data = balls.get_upgraded_data();
            let position = ecs.components.positions.get_mut(ball_e).unwrap();
            let ball_index = ecs.components.balls.get_mut(ball_e).unwrap();

            let angle = Vec2::from_angle(
                TAU / balls_data.amount as f32 * *ball_index as f32 + TAU * balls.rotation_progress,
            )
            .rotate(Vec2::X);

            let ball_distance = 22.;
            let desired_pos = player_pos + angle * ball_distance;

            *position = desired_pos;
        } else {
            ecs.despawn(*ball_e);
        };
    }

    if bullet_data.len() > 0 {
        audio::play_sound(
            &data.audio.shoot,
            PlaySoundParams {
                volume: data.settings.sfx_volume * 0.5,
                ..Default::default()
            },
        );
    }

    for (damage, position, vel, bullet_index) in &bullet_data {
        let bullet_id = spawn_bullet(data, ecs, *position, EntityType::Enemy, *damage, *vel);
        if let Some(bullet_index) = bullet_index {
            ecs.components.balls.insert(bullet_id, *bullet_index);
        }
    }
}
