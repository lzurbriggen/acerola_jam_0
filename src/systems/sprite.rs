use macroquad::prelude::*;

use crate::entity::{
    entities::Entities,
    traits::{Position, Sprite},
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
