use crate::game_data::GameData;

use super::player::Player;

pub trait Enemy {
    fn update(&mut self, data: &mut GameData) -> ();
    fn draw(&self, data: &mut GameData) -> ();
}

pub enum Entity {
    Player(Player),
    Enemy(Box<dyn Enemy>),
}
