use macroquad::prelude::*;

use crate::{
    entity::{entities::Components, entity_id::Entity},
    game_data::GameData,
};

pub fn draw_colliders(data: &GameData, entities: &Vec<Entity>, components: &Components) {
    if !data.debug_collisions {
        return;
    }

    let colliders = entities
        .iter()
        .filter(|e| components.positions.contains_key(e) && components.colliders.contains_key(e))
        .collect::<Vec<&Entity>>();

    for id in &colliders {
        let pos = components.positions.get(&id).unwrap();
        let coll = components.colliders.get(&id).unwrap();

        draw_circle_lines(pos.x, pos.y, coll.radius, 1., BLUE)
    }
}
