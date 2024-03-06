use crate::timer::Timer;

#[derive(Debug, PartialEq)]
pub enum DamageSource {
    Player,
    Enemy,
}

pub struct DamageOnCollision {
    pub source: DamageSource,
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
