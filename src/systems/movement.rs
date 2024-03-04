use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    entity::{entities::Ecs, entity_id::Entity},
    game_data::GameData,
    map::map::Map,
    physics::collision::{resolve_circle_collision, resolve_map_collision, Collision},
};

pub fn move_entities(data: &mut GameData, map: &Map, ecs: &mut Ecs) -> HashMap<Entity, Collision> {
    let moveables = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.velocities.contains_key(e)
    });
    let comps = &mut ecs.components;

    let mut colliders = vec![];
    for moveable in &moveables {
        if let Some(_) = comps.colliders.get(moveable) {
            let other_pos = *comps.positions.get(moveable).unwrap();
            let other_coll = comps.colliders.get(moveable).unwrap();
            colliders.push((*moveable, other_pos, other_coll));
        }
    }

    let mut collisions = HashMap::<Entity, Collision>::new();
    for moveable in &moveables {
        let position = comps.positions.get_mut(moveable).unwrap();
        let velocity = comps.velocities.get_mut(moveable).unwrap();
        let collider = comps.colliders.get(moveable);

        let mut desired_pos = *position + *velocity * get_frame_time();

        if let Some(collider) = collider {
            let (pos, new_collisions) =
                resolve_circle_collision(*moveable, desired_pos, &colliders);
            collisions.extend(new_collisions);
            desired_pos = pos;
            desired_pos = resolve_map_collision(data, map, desired_pos, collider.radius);
        }
        *position = desired_pos;
    }
    collisions
}
