use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{systems::collision::SphereCollider, timer::Timer};

use super::{
    animated_sprite::AnimatedSprite, door::Door, entity_id::Entity, hopper::Hopper,
    player::PlayerData, spawner::Spawner,
};

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
}
