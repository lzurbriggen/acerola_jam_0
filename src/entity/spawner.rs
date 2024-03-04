use macroquad::prelude::*;

use crate::game_data::GameData;

use super::{entities::Ecs, entity_id::Entity};

#[derive(Clone)]
pub struct Spawner {
    pub active: bool,
    pub last_spawn_time: f64,
}

pub fn spawn_spawner(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    ecs.components.positions.insert(id, position);

    let spawner = Spawner {
        active: true,
        last_spawn_time: get_time(),
    };
    ecs.components.spawners.insert(id, spawner);

    ecs.entities.push(id);

    id
}
