use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game_data::GameData, sprite::indexed_sprite::IndexedSprite, systems::collision::CircleCollider,
};

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::{DespawnOnHit, EntityType},
};

#[derive(Debug)]
pub enum Pickup {
    Health(f32),
    AnomalyBig,
    AnomalySmall,
}

pub fn spawn_pickup(data: &mut GameData, position: Vec2, ecs: &mut Ecs, pickup: Pickup) -> Entity {
    let id = data.new_entity();

    let texture = match pickup {
        Pickup::Health(_) => "health",
        Pickup::AnomalyBig => "anomaly_big",
        Pickup::AnomalySmall => "anomaly_small",
    };
    let indexed_sprite = IndexedSprite::new(data, texture, 16, vec2(8., 8.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("noop".to_string(), Animation::new(vec![0], 0., false))]),
    );
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = CircleCollider {
        radius: 3.,
        trigger: true,
    };
    ecs.components.colliders.insert(id, collider);

    ecs.components.positions.insert(id, position);
    ecs.components.velocities.insert(id, Vec2::ZERO);

    ecs.components.pickups.insert(id, pickup);

    ecs.components
        .materials
        .insert(id, "aberration".to_string());

    ecs.components.room_entity.insert(id, ());
    ecs.components
        .despawn_on_hit
        .insert(id, DespawnOnHit(EntityType::Pickup));

    ecs.entities.push(id);
    id
}
