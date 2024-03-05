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
    tags::{Damageable, Health},
};

pub struct PlayerData {
    pub move_speed: f32,
    pub sprite_offset: Vec2,
    pub max_hp: u8,
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

    let player_data = PlayerData {
        move_speed: 72.,
        sprite_offset: vec2(8., 10.),
        max_hp: 3,
    };

    ecs.components.player_data.insert(id, player_data);
    ecs.components.health.insert(id, Health { hp: 3. });

    ecs.components.damageables.insert(
        id,
        Damageable {
            invulnerable_timer: Some(Timer::new(1., false)),
            hit_fx_timer: Some(Timer::new(0.22, false)),
        },
    );

    ecs.components
        .materials
        .insert(id, create_sprite_color_material());

    ecs.entities.push(id);
    id
}
