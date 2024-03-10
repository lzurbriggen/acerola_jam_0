use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game_data::GameData, sprite::indexed_sprite::IndexedSprite, systems::collision::CircleCollider,
    timer::Timer,
};

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Ecs,
    entity_id::Entity,
    tags::{DamageOnCollision, Damageable, EntityType, Health},
};

pub struct Stomper {
    pub attack_timer: Timer,
    pub spit_timer: Timer,
    pub move_speed: f32,
}

pub fn spawn_stomper(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(data, "stomper", 32, vec2(16., 16.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([(
            "walk".to_string(),
            Animation::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 0.13, true),
        )]),
    );
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = CircleCollider {
        radius: 6.,
        trigger: false,
    };
    ecs.components.colliders.insert(id, collider);

    ecs.components.positions.insert(id, position);
    ecs.components.velocities.insert(id, Vec2::ZERO);

    let stomper = Stomper {
        attack_timer: Timer::new(2., false),
        spit_timer: Timer::new(0.36, false),
        move_speed: 40.,
    };
    ecs.components.stompers.insert(id, stomper);

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
        .insert(id, "aberration".to_string());

    ecs.components.room_entity.insert(id, ());

    ecs.entities.push(id);
    id
}
