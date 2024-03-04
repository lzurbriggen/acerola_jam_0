use macroquad::prelude::*;

use crate::{game_data::GameData, systems::traits::SphereCollider};

use super::{entities::Components, entity_id::Entity};

#[derive(Clone)]
pub struct Door {
    pub radius: f32,
}

impl Door {
    pub fn new() -> Self {
        Self { radius: 5. }
    }
}

pub fn spawn_door(
    data: &mut GameData,
    position: Vec2,
    entities: &mut Vec<Entity>,
    components: &mut Components,
) -> Entity {
    let id = data.new_entity();

    components.positions.insert(id, position);

    let collider = SphereCollider { radius: 5. };
    components.colliders.insert(id, collider);

    let door = Door::new();
    components.doors.insert(id, door);

    entities.push(id);
    id
}
