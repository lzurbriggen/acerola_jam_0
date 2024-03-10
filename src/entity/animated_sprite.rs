use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{game_data::GameData, sprite::indexed_sprite::IndexedSprite, timer::Timer};

pub struct Animation {
    pub repeat: bool,
    pub frames: Vec<usize>,
    pub current_frame: usize,
    pub timer: Timer,
    pub completed: bool,
}

impl Animation {
    pub fn new(frames: Vec<usize>, frame_duration: f32, repeat: bool) -> Self {
        Self {
            repeat,
            frames,
            current_frame: 0,
            timer: Timer::new(frame_duration, true),
            completed: false,
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

    pub fn current_animation(&self) -> (&String, &Animation) {
        (
            &self.current_animation,
            self.animations.get(&self.current_animation).unwrap(),
        )
    }

    pub fn current_animation_mut(&mut self) -> (&String, &mut Animation) {
        (
            &self.current_animation,
            self.animations.get_mut(&self.current_animation).unwrap(),
        )
    }

    pub fn update(&mut self) {
        let (_, anim) = self.current_animation_mut();
        anim.timer.update();
        if anim.timer.just_completed() {
            if anim.current_frame + 1 >= anim.frames.len() {
                if !anim.repeat {
                    anim.completed = true;
                    return;
                }
                anim.current_frame = 0;
            } else {
                anim.current_frame += 1;
            }
        }
        anim.completed = false;
    }

    pub fn draw(&self, data: &GameData, position: Vec2, flipped: bool) {
        let (_, anim) = self.current_animation();
        let index = anim.frames[anim.current_frame];
        self.indexed_sprite.draw(data, position, index, flipped)
    }

    pub fn set_animation(&mut self, name: &str) {
        self.current_animation = name.to_string();
        self.current_animation_mut().1.current_frame = 0;
        self.current_animation_mut().1.completed = false;
    }
}
