use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    entity::{entities::Ecs, entity_id::Entity},
    game_data::GameData,
    physics::collision::{resolve_circle_collision, resolve_map_collision, Collision},
};

pub fn move_entities(data: &mut GameData, ecs: &mut Ecs) -> HashMap<(Entity, Entity), Collision> {
    let moveables = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.velocities.contains_key(e)
    });
    let comps = &mut ecs.components;

    let mut colliders = vec![];
    for moveable_e in &moveables {
        if let Some(_) = comps.colliders.get(moveable_e) {
            let other_pos = *comps.positions.get(moveable_e).unwrap();
            let other_coll = comps.colliders.get(moveable_e).unwrap();
            colliders.push((*moveable_e, other_pos, other_coll));
        }
    }

    let mut failed_entities = vec![];

    let mut collisions = HashMap::<(Entity, Entity), Collision>::new();
    for moveable_e in &moveables {
        let position = comps.positions.get_mut(moveable_e).unwrap();
        let velocity = comps.velocities.get(moveable_e).unwrap();
        let collider = comps.colliders.get(moveable_e);

        let mut desired_pos = *position + *velocity * get_frame_time();
        if desired_pos.x.is_nan() || desired_pos.y.is_nan() {
            failed_entities.push(*moveable_e);
            println!("WTF {:?} {:?} {:?}", *position, velocity, get_frame_time());
            continue;
        }

        if let Some(collider) = collider {
            let (pos, new_collisions) =
                resolve_circle_collision(*moveable_e, desired_pos, &colliders);
            collisions.extend(new_collisions);
            desired_pos = pos;
            let (pos, new_collisions) =
                resolve_map_collision(*moveable_e, data, data.current_map(), desired_pos, collider);
            collisions.extend(new_collisions);
            desired_pos = pos;
        }
        *position = desired_pos;
    }

    // TODO: emergency fix to issue that prevent room completion
    for failed_e in failed_entities {
        ecs.despawn(failed_e);
    }

    collisions
}
