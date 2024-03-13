use std::collections::HashMap;

use crate::{
    entity::{
        entities::Ecs,
        entity_id::Entity,
        events::{DamageEvent, DeathEvent},
        impact::{spawn_dust, splatter_blood},
        mirituhg::{self, spawn_mirituhg_death, MiritughState},
        pickup::{spawn_pickup, Pickup},
        projectile::spawn_bullet,
        skull::spawn_skull,
        tags::EntityType,
    },
    game_data::GameData,
    items::weapon::Weapon,
    physics::collision::Collision,
};
use macroquad::{
    audio::{self, PlaySoundParams},
    prelude::*,
};

use super::collision::ColliderType;

pub fn update_damageables(ecs: &mut Ecs) {
    let damageables = ecs.check_components(|e, comps| comps.damageables.contains_key(e));

    for damageable_e in &damageables {
        let damageable = ecs.components.damageables.get_mut(damageable_e).unwrap();

        if let Some(invulnerable_timer) = &mut damageable.invulnerable_timer {
            invulnerable_timer.update();
        }

        if let Some(hit_fx_timer) = &mut damageable.hit_fx_timer {
            hit_fx_timer.update();
        }
    }
}

pub fn flash_on_damage(ecs: &mut Ecs) {
    let damageables = ecs.check_components(|e, comps| {
        comps.damageables.contains_key(e) && comps.materials.contains_key(e)
    });

    for damageable_e in &damageables {
        let damageable = ecs.components.damageables.get_mut(damageable_e).unwrap();
        let material = ecs.components.materials.get_mut(damageable_e).unwrap();

        if let Some(hit_fx_timer) = &mut damageable.hit_fx_timer {
            if !hit_fx_timer.completed() {
                let intensity = hit_fx_timer.progress() * 10. + 1.;
                let mut color = WHITE;
                color.r = intensity;
                color.g = intensity;
                color.b = intensity;
                color.a = (0.5 - hit_fx_timer.progress() % 0.5) * 2.;
                material.set_uniform("color", color);
            } else {
                material.set_uniform("color", WHITE);
            }
        }
    }
}

pub fn apply_damage(data: &mut GameData, ecs: &mut Ecs, damage_events: &mut Vec<DamageEvent>) {
    let damageables = ecs.check_components(|e, comps| {
        comps.damageables.contains_key(e) && comps.health.contains_key(e)
    });

    let mut splatter_positions = vec![];

    for damageable_e in &damageables {
        let damageable = ecs.components.damageables.get_mut(damageable_e).unwrap();
        let health = ecs.components.health.get_mut(damageable_e).unwrap();

        let mut event_indices = damage_events
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if e.target == *damageable_e {
                    return Some(i);
                }
                None
            })
            .collect::<Vec<usize>>();
        event_indices.reverse();

        // TODO: use indices instead
        let event = damage_events.iter().find(|e| e.target == *damageable_e);

        let mut bullets = vec![];

        if let Some(event) = event {
            if let Some(invulnerable_timer) = &mut damageable.invulnerable_timer {
                if invulnerable_timer.completed() {
                    let is_player = ecs.components.player_data.contains_key(damageable_e);
                    let is_enemy = ecs.components.enemies.contains_key(damageable_e);

                    let mut apply_damage = true;
                    if is_player {
                        if let Weapon::Dash(ref dash) = data.weapon {
                            if dash.dashing {
                                apply_damage = false;
                            } else {
                                audio::play_sound(
                                    &data.audio.hit2,
                                    PlaySoundParams {
                                        volume: data.settings.sfx_volume,
                                        ..Default::default()
                                    },
                                );
                            }
                        }
                    }
                    if is_enemy {
                        let coll = ecs.components.colliders.get(&event.source).unwrap();

                        if coll.coll_type == ColliderType::ProjectileWithoutMapCollision {
                            if let Weapon::Balls(ref balls) = data.weapon {
                                let up_data = balls.get_upgraded_data();
                                if up_data.bullets {
                                    let pos = ecs.components.positions.get(&event.target).unwrap();
                                    bullets.push(*pos);
                                }
                            }
                        }
                        if let Weapon::Dash(ref balls) = data.weapon {
                            let up_data = balls.get_upgraded_data();
                            if up_data.bullets {
                                let pos = ecs.components.positions.get(&event.target).unwrap();
                                bullets.push(*pos);
                            }
                        }
                    }

                    if apply_damage {
                        health.hp -= event.damage;
                        if is_player {
                            data.screen_shake.shake(0.25, 8.);
                        }
                    }

                    audio::play_sound(
                        &data.audio.hit,
                        PlaySoundParams {
                            volume: data.settings.sfx_volume,
                            ..Default::default()
                        },
                    );

                    if let Some(position) = ecs.components.positions.get(damageable_e) {
                        splatter_positions.push(*position);
                    }
                    invulnerable_timer.reset();
                    if let Some(hit_fx_timer) = &mut damageable.hit_fx_timer {
                        hit_fx_timer.reset();
                    }
                }
            }

            for index in event_indices {
                damage_events.remove(index);
            }
        }

        let bullet_vel = 50.;
        let bullet_damage = 7.;
        for pos in bullets {
            spawn_bullet(
                data,
                ecs,
                pos + vec2(1., 0.) * 12.,
                EntityType::Enemy,
                bullet_damage,
                vec2(1., 0.) * bullet_vel,
                ColliderType::PlayerProjectile,
            );
            spawn_bullet(
                data,
                ecs,
                pos + vec2(0., 1.) * 12.,
                EntityType::Enemy,
                bullet_damage,
                vec2(0., 1.) * bullet_vel,
                ColliderType::PlayerProjectile,
            );
            spawn_bullet(
                data,
                ecs,
                pos + vec2(-1., 0.) * 12.,
                EntityType::Enemy,
                bullet_damage,
                vec2(-1., 0.) * bullet_vel,
                ColliderType::PlayerProjectile,
            );
            spawn_bullet(
                data,
                ecs,
                pos + vec2(0., -1.) * 12.,
                EntityType::Enemy,
                bullet_damage,
                vec2(0., -1.) * bullet_vel,
                ColliderType::PlayerProjectile,
            );
        }
    }

    for pos in &splatter_positions {
        splatter_blood(data, ecs, *pos);
    }

    damage_events.clear();
}

pub fn damage_on_collision(
    ecs: &Ecs,
    damage_events: &mut Vec<DamageEvent>,
    collisions: &HashMap<(Entity, Entity), Collision>,
) {
    let damageables = ecs.check_components(|e, comps| comps.damageables.contains_key(e));

    for damageable_e in &damageables {
        for ((source, target), _collision) in collisions.iter() {
            if target != damageable_e && source != damageable_e {
                continue;
            }
            for (e1, e2) in [(source, target), (target, source)] {
                if let Some(damage_on_coll) = ecs.components.damage_on_collision.get(e1) {
                    let apply_damage = if ecs.components.player_data.contains_key(e2) {
                        damage_on_coll.source == EntityType::Enemy
                    } else {
                        damage_on_coll.source == EntityType::Player
                    };
                    if apply_damage {
                        damage_events.push(DamageEvent {
                            source: *e1,
                            target: *e2,
                            damage: damage_on_coll.damage,
                        });
                    }
                }
            }
        }
    }
}

pub fn despawn_on_collision(
    data: &mut GameData,
    ecs: &mut Ecs,
    collisions: &HashMap<(Entity, Entity), Collision>,
) {
    let despawn_on_hits = ecs.check_components(|e, comps| comps.despawn_on_hit.contains_key(e));

    for despawn_e in &despawn_on_hits {
        for ((source, target), _collision) in collisions.iter() {
            for (e1, e2) in [(source, target), (target, source)] {
                if e1 == despawn_e {
                    let despawn_on_hit = ecs.components.despawn_on_hit.get(despawn_e).unwrap();
                    // TODO: not safe
                    let position = ecs.components.positions.get(despawn_e).unwrap();
                    // if ecs.components.player_entity.contains_key(e2)
                    //     && despawn_on_hit.0 == EntityType::Player
                    // {
                    //     spawn_dust(data, ecs, *position);
                    //     ecs.despawn(*despawn_e);
                    //     break;
                    // };

                    if let Some(player) = ecs.components.player_data.get_mut(e2) {
                        let up_data = player.get_upgraded_data();
                        if let Some(pickup) = ecs.components.pickups.get(despawn_e) {
                            if match pickup {
                                Pickup::Health(increase) => {
                                    let mut low_hp = false;
                                    if let Some(health) = ecs.components.health.get_mut(e2) {
                                        low_hp = health.hp < up_data.max_hp as f32;
                                        if low_hp {
                                            health.hp =
                                                (health.hp + increase).min(up_data.max_hp as f32);
                                        }
                                    }
                                    low_hp
                                }
                                Pickup::AnomalyBig => {
                                    player.aberration = (player.aberration - 0.1).max(0.);
                                    true
                                }
                                Pickup::AnomalySmall => {
                                    player.aberration = (player.aberration - 0.02).max(0.);
                                    true
                                }
                            } {
                                spawn_dust(data, ecs, *position);
                                ecs.despawn(*despawn_e);
                                audio::play_sound(
                                    &data.audio.confirm2,
                                    PlaySoundParams {
                                        volume: data.settings.sfx_volume * 1.2,
                                        ..Default::default()
                                    },
                                );
                                break;
                            }

                            continue;
                        }
                    }

                    spawn_dust(data, ecs, *position);
                    ecs.despawn(*despawn_e);
                    break;
                }
            }
        }
    }
}

pub fn kill_entities(data: &GameData, ecs: &mut Ecs, death_events: &mut Vec<DeathEvent>) {
    let healthies = ecs.check_components(|e, comps| comps.health.contains_key(e));

    for health_e in &healthies {
        let health = ecs.components.health.get_mut(health_e).unwrap();

        if health.hp <= 0. {
            ecs.despawn(*health_e);
            death_events.push(DeathEvent(*health_e));

            let aberration_increase = ecs.components.aberration_increase.get(health_e);
            if let Some(inc) = aberration_increase {
                let inc = inc * (1. + data.completed_rooms as f32 * 0.37);
                // let inc = inc * (1. + data.completed_rooms as f32 * 0.07);
                let players = ecs.check_components(|e, comps| comps.player_data.contains_key(e));

                for player_e in &players {
                    let player_data = ecs.components.player_data.get_mut(player_e).unwrap();
                    player_data.aberration = (player_data.aberration + inc).clamp(0., 1.);
                }
            }
        }
    }
}

pub fn handle_death(data: &mut GameData, ecs: &mut Ecs, death_events: &Vec<DeathEvent>) {
    let mut skull_positions = vec![];
    let mut pickups = vec![];

    let mut spawn_death = None;
    for ev in death_events {
        let pos = ecs.components.positions.get(&ev.0).unwrap();
        let player = ecs.components.player_entity.get(&ev.0);
        let mirituhg = ecs.components.mirituhg.get_mut(&ev.0);

        if player.is_some() {
            data.dead = true;
            audio::play_sound(
                &data.audio.death2,
                PlaySoundParams {
                    volume: data.settings.sfx_volume,
                    ..Default::default()
                },
            );
            data.death_screen.show();

            for _ in 0..40 {
                skull_positions.push(vec2(rand::gen_range(0., 360.), rand::gen_range(0., 240.)))
            }
        } else if let Some(mirituhg) = mirituhg {
            mirituhg.state = MiritughState::Dead;
            spawn_death = Some(*pos);
        } else {
            let rand = rand::gen_range(0, (12 - data.item_drop_chance_increase).max(4));
            match rand {
                0..=1 => pickups.push((Pickup::Health(1.), *pos)),
                2..=3 => pickups.push((Pickup::AnomalySmall, *pos)),
                4 => pickups.push((Pickup::AnomalyBig, *pos)),
                _ => {}
            }
        }
        audio::play_sound(
            &data.audio.death,
            PlaySoundParams {
                volume: data.settings.sfx_volume,
                ..Default::default()
            },
        );

        skull_positions.push(*pos);
    }

    if let Some(pos) = spawn_death {
        data.screen_shake.shake(2.25, 4.);
        spawn_mirituhg_death(data, pos, ecs);
    }

    for (pickup, pos) in pickups {
        spawn_pickup(data, pos, ecs, pickup);
    }

    for pos in skull_positions {
        spawn_skull(data, ecs, pos);
    }
}
