use macroquad::prelude::*;

use crate::{
    entity::{entities::Components, entity_id::Entity},
    game_data::GameData,
    map::map::Map,
    physics::collision::resolve_map_collision,
};

pub fn move_entities(
    data: &mut GameData,
    map: &Map,
    entities: &Vec<Entity>,
    components: &mut Components,
) {
    let moveables = entities
        .iter()
        .filter(|e| components.positions.contains_key(e) && components.velocities.contains_key(e))
        .collect::<Vec<&Entity>>();

    for moveable in &moveables {
        let position = components.positions.get_mut(moveable).unwrap();
        let velocity = components.velocities.get_mut(moveable).unwrap();
        let collider = components.colliders.get(moveable);

        let desired_pos = *position + *velocity * get_frame_time();

        let mut resulting_position = desired_pos;
        if let Some(collider) = collider {
            resulting_position = resolve_map_collision(data, map, desired_pos, collider.radius);
        }
        *position = resulting_position;
    }
}
