use macroquad::prelude::*;

use crate::game_data::GameData;

use super::{entities::Components, entity_id::Entity};

#[derive(Clone)]
pub struct Spawner {
    pub active: bool,
    pub last_spawn_time: f64,
}

pub fn spawn_spawner(
    data: &mut GameData,
    position: Vec2,
    entities: &mut Vec<Entity>,
    components: &mut Components,
) -> Entity {
    let id = data.new_entity();

    components.positions.insert(id, position);

    let spawner = Spawner {
        active: true,
        last_spawn_time: get_time(),
    };
    components.spawners.insert(id, spawner);

    entities.push(id);

    id
}
