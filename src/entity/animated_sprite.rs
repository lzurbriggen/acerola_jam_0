use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{sprite::indexed_sprite::IndexedSprite, timer::Timer};

pub struct Animation {
    pub repeat: bool,
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: Timer,
}

impl Animation {
    pub fn new(frames: Vec<usize>, frame_duration: f32, repeat: bool) -> Self {
        Self {
            repeat,
            frames,
            current_frame: 0,
            timer: Timer::new(frame_duration, true),
        }
    }
}

pub struct AnimatedSprite {
    pub indexed_sprite: IndexedSprite,
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
}

impl AnimatedSprite {
    pub fn new(indexed_sprite: IndexedSprite, animations: HashMap<String, Animation>) -> Self {
        Self {
            indexed_sprite,
            current_animation: animations.iter().next().unwrap().0.clone(),
            animations,
        }
    }

    pub fn draw(&self, position: Vec2) {
        let anim = self.animations.get(&self.current_animation).unwrap();
        let index = anim.frames[anim.current_frame];
        self.indexed_sprite.draw(position, index)
    }
}
