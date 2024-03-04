use macroquad::texture::Texture2D;

use crate::{
    entity::{entities::Components, entity_id::Entity, hopper::spawn_hopper},
    game_data::GameData,
};

pub fn spawn_creatures(
    data: &mut GameData,
    entities: &mut Vec<Entity>,
    comps: &mut Components,
    hopper_texture: &Texture2D,
) {
    let spawners = entities
        .iter()
        .filter(|e| comps.positions.contains_key(e) && comps.spawners.contains_key(e))
        .collect::<Vec<&Entity>>();

    let mut spawns = vec![];
    for spawner_e in &spawners {
        let spawner = comps.spawners.get_mut(&spawner_e).unwrap();
        let position = comps.positions.get(&spawner_e).unwrap();
        if !spawner.active {
            continue;
        }
        spawns.push(position.clone());
        spawner.active = false;
    }

    for spawn_pos in spawns {
        spawn_hopper(data, hopper_texture.clone(), spawn_pos, entities, comps);
    }
}
