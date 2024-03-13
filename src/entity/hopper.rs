use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game_data::GameData,
    sprite::{flash_material::create_sprite_color_material, indexed_sprite::IndexedSprite},
    systems::collision::{CircleCollider, ColliderType},
    timer::Timer,
};

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::{DamageOnCollision, Damageable, EntityType, Health},
};

pub struct Hopper {
    pub jump_timer: Timer,
    pub jumping: bool,
    pub move_speed: f32,
    pub jump_move_speed: f32,
}

pub fn spawn_hopper(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(data, "hopper", 16, vec2(8., 10.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([
            ("move".to_string(), Animation::new(vec![0, 1], 0.3, true)),
            (
                "jump".to_string(),
                Animation::new(vec![2, 3, 4, 5, 6, 7, 8, 9], 0.12, true),
            ),
        ]),
    );
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = CircleCollider {
        radius: 3.,
        coll_type: ColliderType::Enemy,
    };
    ecs.components.colliders.insert(id, collider);

    ecs.components.positions.insert(id, position);
    ecs.components.velocities.insert(id, Vec2::ZERO);

    let hopper = Hopper {
        jump_timer: Timer::new(2., false),
        jumping: false,
        move_speed: 25.,
        jump_move_speed: 25.,
    };
    ecs.components.hoppers.insert(id, hopper);

    ecs.components.damageables.insert(
        id,
        Damageable {
            invulnerable_timer: Some(Timer::new(0.2, false)),
            hit_fx_timer: Some(Timer::new(0.22, false)),
        },
    );
    ecs.components.health.insert(id, Health { hp: 30. });
    ecs.components.damage_on_collision.insert(
        id,
        DamageOnCollision {
            source: EntityType::Enemy,
            damage: 1.,
        },
    );

    ecs.components
        .materials
        .insert(id, create_sprite_color_material());

    ecs.components.room_entity.insert(id, ());
    ecs.components.enemies.insert(id, ());

    ecs.entities.push(id);
    id
}
