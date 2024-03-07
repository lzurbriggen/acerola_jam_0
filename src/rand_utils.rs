use macroquad::{
    math::{vec2, Vec2},
    prelude::rand,
};

pub fn rand_dir() -> Vec2 {
    vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
}
