use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game_data::GameData, sprite::indexed_sprite::IndexedSprite, systems::collision::SphereCollider,
};

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Components,
    entity_id::Entity,
};

pub struct PlayerData {
    pub move_speed: f32,
    pub sprite_offset: Vec2,
    pub hp: u8,
    pub max_hp: u8,
}

pub fn spawn_player(
    data: &mut GameData,
    texture: Texture2D,
    entities: &mut Vec<Entity>,
    components: &mut Components,
) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture, 16, vec2(8., 10.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("idle".to_string(), Animation::new(vec![0], 0., false))]),
    );
    components.animated_sprites.insert(id, sprite);

    let collider = SphereCollider { radius: 3. };
    components.colliders.insert(id, collider);

    components.positions.insert(id, vec2(180., 120.));
    components.velocities.insert(id, Vec2::ZERO);

    let player_data = PlayerData {
        move_speed: 72.,
        sprite_offset: vec2(8., 10.),
        hp: 3,
        max_hp: 3,
    };

    components.player_data.insert(id, player_data);

    entities.push(id);
    id
}
