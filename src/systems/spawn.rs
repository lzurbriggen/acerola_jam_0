use crate::{
    entity::{
        entities::{Enemy, Entities},
        hopper::Hopper,
    },
    game_data::GameData,
};

pub fn spawn_creatures(data: &GameData, entities: &mut Entities) {
    for spawner in &mut entities.spawners {
        if !spawner.active {
            continue;
        }
        let hopper = Hopper::new(data.sprites.hopper.clone(), spawner.position);
        entities.enemies.push(Enemy::Hopper(hopper));
        spawner.active = false;
    }
}
