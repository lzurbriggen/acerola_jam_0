use macroquad::rand;

use crate::entity::entities::Ecs;

#[derive(Clone, Copy)]
pub enum Enemy {
    Hopper,
}

pub struct Room {
    pub enemies_to_spawn: Vec<Enemy>,
}

impl Room {
    pub fn new(difficulty: f32) -> Room {
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
            enemies_to_spawn: enemies,
        }
    }

    pub fn despawn(&self, ecs: &mut Ecs) {
        let room_entities = ecs.check_components(|e, comps| comps.room_entity.contains_key(e));

        for room_e in room_entities {
            ecs.despawn(room_e);
        }
    }
}
