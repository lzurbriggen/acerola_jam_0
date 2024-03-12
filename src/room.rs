use macroquad::rand;

use crate::entity::entities::Ecs;

#[derive(Debug, Clone, Copy)]
pub enum Enemy {
    Hopper,
    Spitter,
    Stomper,
    Mirituhg,
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
    pub started: bool,
    pub aberration_completed: bool,
    pub completed: bool,
    pub upgrade_chosen: bool,
}

impl Room {
    pub fn new(map_index: usize, difficulty: f32) -> Room {
        let mut remaining_difficulty = difficulty;
        let enemy_values = vec![
            (Enemy::Hopper, 1.),
            (Enemy::Spitter, 2.),
            (Enemy::Stomper, 5.),
        ];

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
            started: false,
            aberration_completed: false,
            completed: false,
            upgrade_chosen: false,
        }
    }

    pub fn check_completed(&mut self, ecs: &Ecs) {
        let enemy_entities = ecs.check_components(|e, comps| comps.enemies.contains_key(e));
        self.completed = self.started && enemy_entities.len() == 0;
    }

    pub fn despawn(&self, ecs: &mut Ecs) {
        let room_entities = ecs.check_components(|e, comps| comps.room_entity.contains_key(e));

        for room_e in room_entities {
            ecs.despawn(room_e);
        }
    }
}
