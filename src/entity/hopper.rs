use macroquad::prelude::*;

use crate::{sprite::indexed_sprite::IndexedSprite, timer::Timer};

use super::traits::{MultiSprite, Position, SphereCollider, TimerProgress};

pub struct Hopper {
    sprite: IndexedSprite,
    pub position: Vec2,
    sprite_offset: Vec2,
    pub collider_radius: f32,
    pub timer: Timer,
    pub jumping: bool,
    pub velocity: Vec2,
    pub move_speed: f32,
    pub jump_move_speed: f32,
}

impl Hopper {
    pub fn new(sprite: IndexedSprite, position: Vec2) -> Self {
        Self {
            sprite,
            position,
            sprite_offset: vec2(8., 8.),
            collider_radius: 3.,
            timer: Timer::new(2., false),
            jumping: false,
            velocity: Vec2::ZERO,
            move_speed: 30.,
            jump_move_speed: 40.,
        }
    }
}

impl MultiSprite for Hopper {
    fn indexed_sprite(&self) -> &IndexedSprite {
        &self.sprite
    }
}

impl Position for Hopper {
    fn position(&self) -> Vec2 {
        self.position
    }
}

impl SphereCollider for Hopper {
    fn radius(&self) -> f32 {
        self.collider_radius
    }
}

impl TimerProgress for Hopper {
    fn update_timers(&mut self) {
        self.timer.update()
    }
}
