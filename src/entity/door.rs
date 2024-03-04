use macroquad::prelude::*;

use crate::{game_data::GameData, systems::collision::SphereCollider};

use super::{entities::Ecs, entity_id::Entity};

#[derive(Clone)]
pub struct Door {
    pub radius: f32,
}

impl Door {
    pub fn new() -> Self {
        Self { radius: 5. }
    }
}

pub fn spawn_door(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    ecs.components.positions.insert(id, position);

    let collider = SphereCollider { radius: 5. };
    ecs.components.colliders.insert(id, collider);

    let door = Door::new();
    ecs.components.doors.insert(id, door);

    ecs.entities.push(id);
    id
}
