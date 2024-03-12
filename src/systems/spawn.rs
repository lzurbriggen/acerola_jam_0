use crate::{
    entity::{
        entities::Ecs, hopper::spawn_hopper, impact::spawn_dust, spitter::spawn_spitter,
        stomper::spawn_stomper,
    },
    game_data::GameData,
    room::Enemy,
};

pub fn spawn_creatures(data: &mut GameData, ecs: &mut Ecs) {
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
        if position.x.is_nan() || position.y.is_nan() {}
        spawns.push(position.clone());
        if data.current_room.enemies_to_spawn.len() == 0 {
            spawner.active = false;
        }
    }

    for spawn_pos in spawns {
        if data.current_room.enemies_to_spawn.len() > 0 {
            match data.current_room.enemies_to_spawn[data.current_room.enemies_to_spawn.len() - 1] {
                Enemy::Hopper => {
                    spawn_hopper(data, spawn_pos, ecs);
                    spawn_dust(data, ecs, spawn_pos);
                }
                Enemy::Spitter => {
                    spawn_spitter(data, spawn_pos, ecs);
                    spawn_dust(data, ecs, spawn_pos);
                }
                Enemy::Stomper => {
                    spawn_stomper(data, spawn_pos, ecs);
                    spawn_dust(data, ecs, spawn_pos);
                }
                _ => todo!(),
            }
            data.current_room.enemies_to_spawn.pop();
        }
    }

    data.current_room.entities_spawned = true;
}
