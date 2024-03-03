use super::{door::Door, hopper::Hopper, player::Player, spawner::Spawner};

pub struct Entities {
    pub player: Player,
    pub doors: Vec<Door>,
    pub spawners: Vec<Spawner>,
    pub enemies: Vec<Enemy>,
}

pub enum Enemy {
    Hopper(Hopper),
}
