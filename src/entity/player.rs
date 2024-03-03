use macroquad::prelude::*;

use crate::game_data::GameData;

use super::traits::{Position, SphereCollider, Sprite};

pub struct Player {
    pub texture: Texture2D,
    pub position: Vec2,
    pub move_speed: f32,
    pub sprite_offset: Vec2,
    pub collider_radius: f32,
    pub hp: u8,
    pub max_hp: u8,
}

impl Player {
    pub fn new(texture: Texture2D, _data: &GameData) -> Self {
        Self {
            texture,
            position: vec2(180., 120.),
            move_speed: 72.,
            sprite_offset: vec2(8., 10.),
            collider_radius: 3.,
            hp: 3,
            max_hp: 3,
        }
    }
}

impl SphereCollider for Player {
    fn radius(&self) -> f32 {
        self.collider_radius
    }
}

impl Position for Player {
    fn position(&self) -> Vec2 {
        self.position
    }
}

impl Sprite for Player {
    fn texture_and_offset(&self) -> (&Texture2D, Vec2) {
        (&self.texture, self.sprite_offset)
    }
}
