use macroquad::prelude::*;

use crate::{
    entity::{
        entities::Entities,
        traits::{Position, SphereCollider},
    },
    game_data::GameData,
};

pub fn draw_colliders(data: &GameData, entities: &Entities) {
    if !data.debug_collisions {
        return;
    }

    draw_circle(&entities.player);
    for door in &entities.doors {
        draw_circle(door);
    }
}

fn draw_circle<T: Position + SphereCollider>(collider: &T) {
    let pos = collider.position();
    let collider = collider.radius();
    draw_circle_lines(pos.x, pos.y, collider, 1., BLUE)
}
