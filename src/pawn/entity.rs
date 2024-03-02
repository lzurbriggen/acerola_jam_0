use crate::game_data::GameData;

pub trait Entity {
    fn update(&mut self, data: &mut GameData) -> ();
    fn draw(&self, data: &mut GameData) -> ();
}
