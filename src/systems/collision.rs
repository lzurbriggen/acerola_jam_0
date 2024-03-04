use macroquad::prelude::*;

use crate::{entity::entities::Ecs, game_data::GameData};

pub struct SphereCollider {
    pub radius: f32,
}

pub fn draw_colliders(data: &GameData, ecs: &Ecs) {
    if !data.debug_collisions {
        return;
    }

    let colliders = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.colliders.contains_key(e)
    });

    for id in &colliders {
        let pos = ecs.components.positions.get(&id).unwrap();
        let coll = ecs.components.colliders.get(&id).unwrap();

        draw_circle_lines(pos.x, pos.y, coll.radius, 1., BLUE)
    }
}
