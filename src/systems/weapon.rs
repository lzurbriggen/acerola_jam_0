use std::f32::consts::TAU;

use macroquad::{
    audio::{self, PlaySoundParams},
    math::{vec2, Vec2},
    time::{get_frame_time, get_time},
};

use crate::{
    entity::{entities::Ecs, projectile::spawn_bullet, tags::EntityType},
    game_data::GameData,
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
        player_pos = *player_position;
        match &mut data.weapon {
            Weapon::Launcher(ref mut shooter) => {
                shooter.shoot_timer.update();
                if shooter.shoot_timer.just_completed() {
                    bullet_data.push((
                        shooter.damage,
                        *player_position + vec2(3., 0.),
                        vec2(160., 0.),
                        None,
                    ));
                    bullet_data.push((
                        shooter.damage,
                        *player_position + vec2(-3., 0.),
                        vec2(-160., 0.),
                        None,
                    ));
                    bullet_data.push((
                        shooter.damage,
                        *player_position + vec2(0., 3.),
                        vec2(0., 160.),
                        None,
                    ));
                    bullet_data.push((
                        shooter.damage,
                        *player_position + vec2(0., -3.),
                        vec2(0., -160.),
                        None,
                    ));
                }
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
            Weapon::Dash(ref mut dash) => {}
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
