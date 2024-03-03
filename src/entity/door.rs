use macroquad::prelude::*;

use super::traits::{Position, SphereCollider};

pub struct Door {
    pub position: Vec2,
    pub radius: f32,
}

impl Door {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            radius: 5.,
        }
    }
}

impl SphereCollider for Door {
    fn radius(&self) -> f32 {
        self.radius
    }
}

impl Position for Door {
    fn position(&self) -> Vec2 {
        self.position
    }
}
