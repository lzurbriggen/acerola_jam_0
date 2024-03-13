use macroquad::prelude::*;

use crate::{entity::entities::Ecs, game_data::GameData};

#[derive(Debug, PartialEq)]
pub enum ColliderType {
    Projectile,
    PlayerProjectile,
    ProjectileWithoutMapCollision,
    Pickup,
    Player,
    Enemy,
    Map,
}

impl ColliderType {
    // TODO: use collision matrix
    pub fn should_collide(&self, other: &ColliderType) -> bool {
        match self {
            ColliderType::Projectile => match other {
                ColliderType::PlayerProjectile
                | ColliderType::ProjectileWithoutMapCollision
                | ColliderType::Player
                | ColliderType::Map => true,
                _ => false,
            },
            ColliderType::PlayerProjectile => match other {
                ColliderType::Projectile | ColliderType::Enemy | ColliderType::Map => true,
                _ => false,
            },
            ColliderType::ProjectileWithoutMapCollision => match other {
                ColliderType::Projectile | ColliderType::Enemy => true,
                _ => false,
            },
            ColliderType::Pickup => match other {
                ColliderType::Player => true,
                _ => false,
            },
            ColliderType::Player => match other {
                ColliderType::Enemy
                | ColliderType::Pickup
                | ColliderType::Projectile
                | ColliderType::Map => true,
                _ => false,
            },
            ColliderType::Enemy => match other {
                ColliderType::Player
                | ColliderType::Enemy
                | ColliderType::Projectile
                | ColliderType::ProjectileWithoutMapCollision
                | ColliderType::Map => true,
                _ => false,
            },
            ColliderType::Map => match other {
                ColliderType::Enemy | ColliderType::Player | ColliderType::Projectile => true,
                _ => false,
            },
        }
    }
}

pub struct CircleCollider {
    pub radius: f32,
    pub coll_type: ColliderType,
}

pub fn draw_colliders(data: &GameData, ecs: &Ecs) {
    if !data.debug_collisions {
        return;
    }

    let colliders = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.colliders.contains_key(e)
    });

    for id in &colliders {
        let pos = ecs.components.positions.get(&id).unwrap();
        let coll = ecs.components.colliders.get(&id).unwrap();

        draw_circle_lines(pos.x, pos.y, coll.radius, 1., BLUE)
    }
}
