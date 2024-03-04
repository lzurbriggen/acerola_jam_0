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
