use crate::entity::entities::Entities;

pub fn spawn_creatures(entities: &mut Entities) {
    for spawner in &entities.spawners {
        if !spawner.active {
            continue;
        }
    }
}
