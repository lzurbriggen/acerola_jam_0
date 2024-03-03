use macroquad::prelude::*;

use crate::sprite::indexed_sprite::IndexedSprite;

pub trait SphereCollider {
    fn radius(&self) -> f32;
}

pub trait Position {
    fn position(&self) -> Vec2;
}

pub trait Sprite {
    fn texture_and_offset(&self) -> (&Texture2D, Vec2);
}

pub trait MultiSprite {
    fn indexed_sprite(&self) -> &IndexedSprite;
}

pub trait TimerProgress {
    fn update_timers(&mut self);
}
