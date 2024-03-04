use macroquad::prelude::*;

use crate::{entity::animated_sprite::AnimatedSprite, sprite::indexed_sprite::IndexedSprite};

pub struct SphereCollider {
    pub radius: f32,
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

pub trait SpriteAnimation {
    fn animated_sprite(&self) -> &AnimatedSprite;
    fn animated_sprite_mut(&mut self) -> &mut AnimatedSprite;
}

pub trait Timeable {
    fn update_timers(&mut self);
}

pub trait Moveable {
    fn move_entity(&mut self);
}

pub trait Drawable {
    fn draw(&self);
}
