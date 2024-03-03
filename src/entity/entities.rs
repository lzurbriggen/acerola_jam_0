use super::{door::Door, player::Player};

pub struct Entities {
    pub player: Player,
    pub doors: Vec<Door>,
}
