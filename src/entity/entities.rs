use super::{
    animated_sprite::AnimatedSprite,
    door::Door,
    entity_id::Entity,
    hopper::Hopper,
    player::PlayerData,
    spawner::Spawner,
    tags::{DamageOnCollision, Health},
};
use crate::{systems::collision::SphereCollider, timer::Timer};
use macroquad::{material::Material, math::Vec2};
use std::collections::HashMap;

pub type ComponentVec<T> = HashMap<Entity, T>;

#[derive(Default)]
pub struct Components {
    pub player_data: ComponentVec<PlayerData>,
    pub timers: ComponentVec<Timer>,
    pub animated_sprites: ComponentVec<AnimatedSprite>,
    pub colliders: ComponentVec<SphereCollider>,
    pub doors: ComponentVec<Door>,
    pub positions: ComponentVec<Vec2>,
    pub velocities: ComponentVec<Vec2>,
    pub spawners: ComponentVec<Spawner>,
    pub hoppers: ComponentVec<Hopper>,
    pub damage_on_collision: ComponentVec<DamageOnCollision>,
    pub health: ComponentVec<Health>,
    pub materials: ComponentVec<Material>,
}

#[derive(Default)]
pub struct Ecs {
    pub entities: Vec<Entity>,
    pub components: Components,
}

impl Ecs {
    pub fn check_components<P>(&self, predicate: P) -> Vec<Entity>
    where
        P: Fn(&Entity, &Components) -> bool,
    {
        self.entities
            .iter()
            .filter(|e| predicate(e, &self.components))
            .map(|e| *e)
            .collect::<Vec<Entity>>()
    }
}
