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

#[derive(Debug, PartialEq)]
pub enum MiritughState {
    Idle,
    Spawn,
    ShootTransition,
    Shoot,
    Dead,
}

pub struct Mirituhg {
    pub move_speed: f32,
    pub shoot_move_speed: f32,
    pub max_hp: f32,

    pub state: MiritughState,
    pub previously_spawned: bool,
    pub next_attack_timer: Timer,
    pub shoot_timer: Timer,

    pub spawn_1_timer: Timer,
    pub spawn_2_timer: Timer,
    pub spawn_3_timer: Timer,
    pub spawning: bool,

    pub next_move_timer: Timer,
    pub target_pos: Vec2,

    pub shoot_rotation: f32,
}

impl Mirituhg {
    pub fn update(&mut self) {
        self.next_attack_timer.update();
        self.shoot_timer.update();
        self.spawn_1_timer.update();
        self.spawn_2_timer.update();
        self.spawn_3_timer.update();
        self.next_move_timer.update();
        self.shoot_rotation += 1.25 * get_frame_time();
    }
}

pub fn spawn_mirituhg(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(data, "mirituhg", 64, vec2(32., 32.));
    let mut sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([
            (
                "idle".to_string(),
                Animation::new(vec![0, 1, 2, 3, 4], 0.15, true),
            ),
            (
                "spawn".to_string(),
                Animation::new(
                    vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
                    0.15,
                    false,
                ),
            ),
            (
                "shoot_transition".to_string(),
                Animation::new(
                    vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33],
                    0.15,
                    false,
                ),
            ),
            (
                "death".to_string(),
                Animation::new(
                    vec![
                        35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                    ],
                    0.15,
                    false,
                ),
            ),
        ]),
    );
    sprite.set_animation("idle");
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = CircleCollider {
        radius: 12.,
        coll_type: ColliderType::Enemy,
    };
    ecs.components.colliders.insert(id, collider);
    ecs.components.positions.insert(id, position);
    ecs.components.velocities.insert(id, Vec2::ZERO);

    let hp = 730.;
    let mirituhg = Mirituhg {
        move_speed: 40.,
        shoot_move_speed: 25.,
        max_hp: hp,
        state: MiritughState::Idle,
        previously_spawned: false,
        spawning: false,
        next_attack_timer: Timer::new(4., false),
        shoot_timer: Timer::new(0.15, false),
        spawn_1_timer: Timer::new(0.15 * 5., false),
        spawn_2_timer: Timer::new(0.15 * 7., false),
        spawn_3_timer: Timer::new(0.15 * 9., false),
        next_move_timer: Timer::new(2.5, true),
        target_pos: vec2(180., 120.),
        shoot_rotation: 0.,
    };
    ecs.components.mirituhg.insert(id, mirituhg);

    ecs.components.damageables.insert(
        id,
        Damageable {
            invulnerable_timer: Some(Timer::new(0.2, false)),
            hit_fx_timer: Some(Timer::new(0.22, false)),
        },
    );
    ecs.components.health.insert(id, Health { hp });
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

pub fn spawn_mirituhg_death(data: &mut GameData, position: Vec2, ecs: &mut Ecs) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(data, "mirituhg", 64, vec2(32., 32.));
    let mut sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([(
            "death".to_string(),
            Animation::new(
                vec![
                    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                ],
                0.15,
                false,
            ),
        )]),
    );
    sprite.set_animation("death");
    ecs.components.animated_sprites.insert(id, sprite);

    let collider = CircleCollider {
        radius: 12.,
        coll_type: ColliderType::Enemy,
    };
    ecs.components.colliders.insert(id, collider);
    ecs.components.positions.insert(id, position);

    // ecs.components
    //     .materials
    //     .insert(id, create_sprite_color_material());
    ecs.components.room_entity.insert(id, ());
    ecs.components.enemies.insert(id, ());
    ecs.components.mirituhg_death.insert(id, ());

    ecs.entities.push(id);
    id
}
