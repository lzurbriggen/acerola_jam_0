use crate::{
    game_data::GameData,
    sprite::{flash_material::create_sprite_color_material, indexed_sprite::IndexedSprite},
    systems::collision::SphereCollider,
    timer::Timer,
};
use macroquad::prelude::*;
use std::collections::HashMap;

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::Health,
};

pub struct PlayerData {
    pub move_speed: f32,
    pub sprite_offset: Vec2,
    pub max_hp: u8,
    pub invulnerable_timer: Timer,
    pub hit_timer: Timer,
}

pub fn spawn_player(data: &mut GameData, texture: Texture2D, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture.clone(), 16, vec2(8., 10.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("idle".to_string(), Animation::new(vec![0], 0., false))]),
    );
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = SphereCollider { radius: 3. };
    ecs.components.colliders.insert(id, collider);

    ecs.components.positions.insert(id, vec2(180., 120.));
    ecs.components.velocities.insert(id, Vec2::ZERO);

    let invulnerable_timer = Timer::new(1., false);
    let player_data = PlayerData {
        move_speed: 72.,
        sprite_offset: vec2(8., 10.),
        max_hp: 3,
        invulnerable_timer,
        hit_timer: Timer::new(0.22, false),
    };

    ecs.components.player_data.insert(id, player_data);
    ecs.components.health.insert(id, Health { hp: 3. });

    ecs.components
        .materials
        .insert(id, create_sprite_color_material());

    ecs.entities.push(id);
    id
}
