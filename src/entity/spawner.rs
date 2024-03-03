use super::traits::Position;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Spawner {
    pub position: Vec2,
    pub active: bool,
    pub last_spawn_time: f64,
}

impl Spawner {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            active: true,
            last_spawn_time: get_time(),
        }
    }
}

impl Position for Spawner {
    fn position(&self) -> Vec2 {
        self.position
    }
}
