use macroquad::prelude::*;

use crate::entity::{
    entities::{Enemy, Entities},
    traits::{MultiSprite, Position, Sprite},
};

pub fn draw_simple_sprites(entities: &Entities) {
    draw_sprite(&entities.player);
}

fn draw_sprite<T: Position + Sprite>(sprite: &T) {
    let (texture, offset) = sprite.texture_and_offset();
    let position = sprite.position() - offset;
    draw_texture_ex(
        &texture,
        position.x,
        position.y,
        WHITE,
        DrawTextureParams {
            ..Default::default()
        },
    );
}

pub fn draw_multi_sprites(entities: &Entities) {
    for enemy in &entities.enemies {
        match enemy {
            Enemy::Hopper(hopper) => draw_multi_sprite(hopper),
        }
    }
}

fn draw_multi_sprite<T: Position + MultiSprite>(sprite: &T) {
    sprite.indexed_sprite().draw(sprite.position(), 0);
}
