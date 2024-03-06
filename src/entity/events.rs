use super::entity_id::Entity;

pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub damage: f32,
}

pub struct DeathEvent(pub Entity);
