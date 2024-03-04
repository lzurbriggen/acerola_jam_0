use macroquad::prelude::*;

use crate::entity::{entities::Components, entity_id::Entity};

use super::traits::{MultiSprite, Position, Sprite};

// pub fn draw_simple_sprites(entities: &Entities) {
//     // draw_sprite(&entities.player);
// }

// fn draw_sprite<T: Position + Sprite>(sprite: &T) {
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

// pub fn draw_multi_sprites(entities: &Entities) {
//     for enemy in &entities.enemies {
//         match enemy {
//             _ => {}
//         }
//     }
// }

// fn draw_multi_sprite<T: Position + MultiSprite>(sprite: &T) {
//     sprite.indexed_sprite().draw(sprite.position(), 0);
// }

pub fn update_animated_sprites(entities: &mut Vec<Entity>, comps: &mut Components) {
    let sprites = entities
        .iter()
        .filter(|e| comps.animated_sprites.contains_key(e))
        .collect::<Vec<&Entity>>();

    for sprite in &sprites {
        let sprite = comps.animated_sprites.get_mut(&sprite).unwrap();
        sprite.update();
    }
}

pub fn draw_animated_sprites(entities: &Vec<Entity>, comps: &Components) {
    let sprites = entities
        .iter()
        .filter(|e| comps.positions.contains_key(e) && comps.animated_sprites.contains_key(e))
        .collect::<Vec<&Entity>>();

    for sprite in &sprites {
        let position = comps.positions.get(&sprite).unwrap();
        let sprite = comps.animated_sprites.get(&sprite).unwrap();
        sprite.draw(*position);
    }
}
