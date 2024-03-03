use macroquad::prelude::*;

pub trait SphereCollider {
    fn radius(&self) -> f32;
}

pub trait Position {
    fn position(&self) -> Vec2;
}

pub trait Sprite {
    fn texture_and_offset(&self) -> (&Texture2D, Vec2);
}
