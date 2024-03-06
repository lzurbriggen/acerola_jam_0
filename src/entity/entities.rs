use super::{
    animated_sprite::AnimatedSprite,
    door::Door,
    entity_id::Entity,
    hopper::Hopper,
    player::PlayerData,
    spawner::Spawner,
    tags::{DamageOnCollision, Damageable, DespawnOnAnimEnd, Health},
};
use crate::{systems::collision::SphereCollider, timer::Timer};
use macroquad::{material::Material, math::Vec2};
use std::collections::HashMap;

pub type ComponentColl<T> = HashMap<Entity, T>;

#[derive(Default)]
pub struct Components {
    pub player_data: ComponentColl<PlayerData>,
    pub timers: ComponentColl<Timer>,
    pub animated_sprites: ComponentColl<AnimatedSprite>,
    pub colliders: ComponentColl<SphereCollider>,
    pub doors: ComponentColl<Door>,
    pub positions: ComponentColl<Vec2>,
    pub velocities: ComponentColl<Vec2>,
    pub spawners: ComponentColl<Spawner>,
    pub hoppers: ComponentColl<Hopper>,
    pub damage_on_collision: ComponentColl<DamageOnCollision>,
    pub health: ComponentColl<Health>,
    pub materials: ComponentColl<Material>,
    pub damageables: ComponentColl<Damageable>,
    pub despawn_on_anim_end: ComponentColl<DespawnOnAnimEnd>,
}

#[derive(Default)]
pub struct Ecs {
    pub entities: Vec<Entity>,
    pub components: Components,
    pub marked_for_despawn: Vec<Entity>,
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

    pub fn despawn(&mut self, entity: Entity) {
        self.marked_for_despawn.push(entity);
    }

    pub fn remove_all_components(&mut self, entity: &Entity) {
        // TODO: lol
        self.components.player_data.remove(entity);
        self.components.timers.remove(entity);
        self.components.animated_sprites.remove(entity);
        self.components.colliders.remove(entity);
        self.components.doors.remove(entity);
        self.components.positions.remove(entity);
        self.components.velocities.remove(entity);
        self.components.spawners.remove(entity);
        self.components.hoppers.remove(entity);
        self.components.damage_on_collision.remove(entity);
        self.components.health.remove(entity);
        self.components.materials.remove(entity);
        self.components.damageables.remove(entity);
        self.components.despawn_on_anim_end.remove(entity);
    }
}
