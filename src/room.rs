use macroquad::rand;

use crate::entity::entities::Ecs;

#[derive(Clone, Copy)]
pub enum Enemy {
    Hopper,
}

#[derive(Clone, Copy)]
pub enum Item {
    Health,
    SuperHealth,
    AberrationRelief,
}

pub struct Room {
    pub map_index: usize,
    pub enemies_to_spawn: Vec<Enemy>,
    pub items_to_spawn: Vec<Item>,
    pub completed: bool,
    pub aberration_completed: bool,
}

impl Room {
    pub fn new(maps_len: usize, difficulty: f32) -> Room {
        let map_index = rand::gen_range(0, maps_len);

        let mut remaining_difficulty = difficulty;
        let enemy_values = vec![(Enemy::Hopper, 1., Enemy::Hopper, 2.)];

        let mut enemies = vec![];

        while remaining_difficulty > 1. {
            let rand_index = rand::gen_range(0, enemy_values.len());
            let enemy = enemy_values[rand_index];
            if enemy.1 > remaining_difficulty {
                continue;
            }
            enemies.push(enemy.0);
            remaining_difficulty -= enemy.1;
        }

        Room {
            map_index,
            enemies_to_spawn: enemies,
            items_to_spawn: vec![],
            completed: false,
            aberration_completed: false,
        }
    }

    pub fn despawn(&self, ecs: &mut Ecs) {
        let room_entities = ecs.check_components(|e, comps| comps.room_entity.contains_key(e));

        for room_e in room_entities {
            ecs.despawn(room_e);
        }
    }
}
