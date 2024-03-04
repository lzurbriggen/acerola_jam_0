use super::traits::{MultiSprite, Position, Sprite, SpriteAnimation};
use macroquad::prelude::*;

// pub fn query_components<T>() -> T {}

// pub fn draw_entities(entities: &Entities) {
//     draw_sprite(&entities.player.texture, entities.player.position);
// }

// pub fn draw_simple_sprites(entities: &Entities) {
//     draw_sprite(&entities.player.texture, entities.player.position);
// }

// fn draw_sprite(sprite: &Texture2D, position: Vec2) {
//     let (texture, offset) = sprite.texture_and_offset();
//     let position = sprite.position() - offset;
//     draw_texture_ex(
//         &texture,
//         position.x,
//         position.y,
//         WHITE,
//         DrawTextureParams {
//             ..Default::default()
//         },
//     );
// }
// fn draw_multi_sprite<T: Position + MultiSprite>(sprite: &T) {
//     sprite.indexed_sprite().draw(sprite.position(), 0);
// }

// fn draw_animated_sprite<T: Position + SpriteAnimation>(sprite: &T) {
//     sprite.animated_sprite().draw(sprite.position());
// }
