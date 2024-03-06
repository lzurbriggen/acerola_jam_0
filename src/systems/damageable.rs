use std::collections::HashMap;

use crate::{
    entity::{
        entities::Ecs,
        entity_id::Entity,
        events::{DamageEvent, DeathEvent},
        skull::spawn_skull,
        tags::EntityType,
    },
    game_data::GameData,
    physics::collision::Collision,
};
use macroquad::prelude::*;

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

pub fn apply_damage(ecs: &mut Ecs, damage_events: &mut Vec<DamageEvent>) {
    let damageables = ecs.check_components(|e, comps| {
        comps.damageables.contains_key(e) && comps.health.contains_key(e)
    });

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

        if let Some(event) = event {
            if let Some(invulnerable_timer) = &mut damageable.invulnerable_timer {
                if invulnerable_timer.completed() {
                    health.hp -= event.damage;
                    println!("{:?}", health.hp);
                    invulnerable_timer.reset();
                    if let Some(hit_fx_timer) = &mut damageable.hit_fx_timer {
                        hit_fx_timer.reset();
                    }
                }
            }

            // for index in event_indices {
            //     damage_events.remove(index);
            // }
        }
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
            if let Some(damage_on_coll) = ecs.components.damage_on_collision.get(source) {
                let apply_damage = if ecs.components.player_data.contains_key(target) {
                    damage_on_coll.source == EntityType::Enemy
                } else {
                    damage_on_coll.source == EntityType::Player
                };
                if apply_damage {
                    damage_events.push(DamageEvent {
                        source: *source,
                        target: *target,
                        damage: damage_on_coll.damage,
                    });
                }
            }
            if let Some(damage_on_coll) = ecs.components.damage_on_collision.get(target) {
                let apply_damage = if ecs.components.player_data.contains_key(source) {
                    damage_on_coll.source == EntityType::Enemy
                } else {
                    damage_on_coll.source == EntityType::Player
                };

                if apply_damage {
                    damage_events.push(DamageEvent {
                        source: *target,
                        target: *source,
                        damage: damage_on_coll.damage,
                    });
                }
            }
        }
    }
}

pub fn despawn_on_collision(ecs: &mut Ecs, collisions: &HashMap<(Entity, Entity), Collision>) {
    let despawn_on_hits = ecs.check_components(|e, comps| comps.despawn_on_hit.contains_key(e));

    for despawn_e in &despawn_on_hits {
        for ((source, target), _collision) in collisions.iter() {
            if source == despawn_e {
                let despawn_on_hit = ecs.components.despawn_on_hit.get(despawn_e).unwrap();
                if ecs.components.player_entity.contains_key(target)
                    && despawn_on_hit.0 == EntityType::Player
                {
                    ecs.despawn(*despawn_e);
                    break;
                };
                if !ecs.components.player_entity.contains_key(target)
                    && despawn_on_hit.0 == EntityType::Enemy
                {
                    ecs.despawn(*despawn_e);
                    break;
                };
            }

            if target == despawn_e {
                let despawn_on_hit = ecs.components.despawn_on_hit.get(despawn_e).unwrap();
                if ecs.components.player_entity.contains_key(source)
                    && despawn_on_hit.0 == EntityType::Player
                {
                    ecs.despawn(*despawn_e);
                    break;
                };
                if !ecs.components.player_entity.contains_key(source)
                    && despawn_on_hit.0 == EntityType::Enemy
                {
                    ecs.despawn(*despawn_e);
                    break;
                };
            }
        }
    }
}

pub fn kill_entities(ecs: &mut Ecs, death_events: &mut Vec<DeathEvent>) {
    let healthies = ecs.check_components(|e, comps| comps.health.contains_key(e));

    for health_e in &healthies {
        let health = ecs.components.health.get_mut(health_e).unwrap();

        if health.hp <= 0. {
            ecs.despawn(*health_e);
            death_events.push(DeathEvent(*health_e));
        }
    }
}

pub fn handle_enemy_death(
    data: &mut GameData,
    skull_texture: Texture2D,
    ecs: &mut Ecs,
    death_events: &Vec<DeathEvent>,
) {
    for ev in death_events {
        let pos = ecs.components.positions.get(&ev.0).unwrap();
        if ecs.components.hoppers.contains_key(&ev.0) {
            spawn_skull(data, skull_texture.clone(), ecs, *pos);
        }
    }
}
