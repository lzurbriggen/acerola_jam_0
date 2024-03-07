use crate::{game_data::GameData, sprite::indexed_sprite::IndexedSprite};
use macroquad::prelude::*;
use std::collections::HashMap;

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::DespawnOnAnimEnd,
};

pub fn spawn_dust(
    data: &mut GameData,
    texture: Texture2D,
    ecs: &mut Ecs,
    position: Vec2,
) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture.clone(), 16, vec2(8., 8.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([(
            "dust".to_string(),
            Animation::new(vec![0, 1, 2, 3, 4, 5], 0.07, false),
        )]),
    );
    ecs.components.animated_sprites.insert(id, sprite);
    ecs.components.positions.insert(id, position);
    ecs.components
        .despawn_on_anim_end
        .insert(id, DespawnOnAnimEnd);
    ecs.components.room_entity.insert(id, ());

    ecs.entities.push(id);
    id
}

pub fn spawn_blood(
    data: &mut GameData,
    texture: Texture2D,
    ecs: &mut Ecs,
    position: Vec2,
) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture.clone(), 16, vec2(8., 8.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("static".to_string(), Animation::new(vec![0], 0., false))]),
    );
    ecs.components.animated_sprites.insert(id, sprite);
    ecs.components.positions.insert(id, position);
    ecs.components.room_entity.insert(id, ());

    ecs.entities.push(id);
    id
}
