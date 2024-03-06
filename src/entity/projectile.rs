use crate::{
    game_data::GameData, sprite::indexed_sprite::IndexedSprite, systems::collision::CircleCollider,
};
use macroquad::prelude::*;
use std::collections::HashMap;

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::{DamageOnCollision, DespawnOnAnimEnd, DespawnOnHit, EntityType},
};

pub fn spawn_bullet(
    data: &mut GameData,
    texture: Texture2D,
    ecs: &mut Ecs,
    position: Vec2,
    target: EntityType,
    damage: f32,
    velocity: Vec2,
) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture.clone(), 16, vec2(8., 8.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("idle".to_string(), Animation::new(vec![0], 4., false))]),
    );
    ecs.components.animated_sprites.insert(id, sprite);
    ecs.components.positions.insert(id, position);
    ecs.components
        .despawn_on_hit
        .insert(id, DespawnOnHit(target));
    ecs.components
        .despawn_on_anim_end
        .insert(id, DespawnOnAnimEnd);
    ecs.components.colliders.insert(
        id,
        CircleCollider {
            radius: 3.,
            trigger: true,
        },
    );
    ecs.components.damage_on_collision.insert(
        id,
        DamageOnCollision {
            damage,
            source: EntityType::Player,
        },
    );
    ecs.components.velocities.insert(id, velocity);
    ecs.components.player_entity.insert(id, ());

    ecs.entities.push(id);
    id
}
