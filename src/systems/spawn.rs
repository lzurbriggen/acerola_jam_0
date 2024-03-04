use macroquad::texture::Texture2D;

use crate::{
    entity::{entities::Ecs, hopper::spawn_hopper},
    game_data::GameData,
};

pub fn spawn_creatures(data: &mut GameData, ecs: &mut Ecs, hopper_texture: &Texture2D) {
    let spawners = ecs.check_components(|e, comps| {
        comps.positions.contains_key(e) && comps.spawners.contains_key(e)
    });

    let mut spawns = vec![];
    for spawner_e in &spawners {
        let spawner = ecs.components.spawners.get_mut(&spawner_e).unwrap();
        let position = ecs.components.positions.get(&spawner_e).unwrap();
        if !spawner.active {
            continue;
        }
        spawns.push(position.clone());
        spawner.active = false;
    }

    for spawn_pos in spawns {
        spawn_hopper(data, hopper_texture.clone(), spawn_pos, ecs);
    }
}
