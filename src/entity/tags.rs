use crate::timer::Timer;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EntityType {
    Player,
    Enemy,
    Pickup,
}

pub struct DamageOnCollision {
    pub source: EntityType,
    pub damage: f32,
}

pub struct Health {
    pub hp: f32,
}

pub struct Damageable {
    pub invulnerable_timer: Option<Timer>,
    pub hit_fx_timer: Option<Timer>,
}

pub struct DespawnOnAnimEnd;
pub struct DespawnOnHit(pub EntityType);
