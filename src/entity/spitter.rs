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

pub struct Spitter {
    pub attack_timer: Timer,
    pub spit_timer: Timer,
}

pub fn spawn_spitter(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(data, "spitter", 16, vec2(8., 10.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([
            (
                "idle".to_string(),
                Animation::new(vec![0, 1, 2, 3], 0.3, true),
            ),
            (
                "spit".to_string(),
                Animation::new(vec![4, 5, 6, 7], 0.12, false),
            ),
        ]),
    );
    ecs.components.animated_sprites.insert(id, sprite);
    ecs.components.flip_to_player.insert(id, ());

    let collider = CircleCollider {
        radius: 5.,
        coll_type: ColliderType::Enemy,
    };
    ecs.components.colliders.insert(id, collider);

    ecs.components.positions.insert(id, position);
    ecs.components.velocities.insert(id, Vec2::ZERO);

    let spitter = Spitter {
        attack_timer: Timer::new(2., false),
        spit_timer: Timer::new(0.36, false),
    };
    ecs.components.spitters.insert(id, spitter);

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
    ecs.components.aberration_increase.insert(id, 0.003);

    ecs.entities.push(id);
    id
}
