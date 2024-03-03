use super::{door::Door, player::Player, spawner::Spawner};

pub struct Entities {
    pub player: Player,
    pub doors: Vec<Door>,
    pub spawners: Vec<Spawner>,
}
