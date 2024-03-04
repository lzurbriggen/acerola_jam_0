use std::cmp::Ordering;

use crate::entity::{entities::Components, entity_id::Entity};

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
    let mut sprites = entities
        .iter()
        .filter(|e| comps.positions.contains_key(e) && comps.animated_sprites.contains_key(e))
        .collect::<Vec<&Entity>>();

    sprites.sort_by(|a, b| {
        let a_pos = comps.positions.get(&a).unwrap();
        let b_pos = comps.positions.get(&b).unwrap();
        if a_pos.y < b_pos.y {
            Ordering::Less
        } else if a_pos.y > b_pos.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for sprite in &sprites {
        let position = comps.positions.get(&sprite).unwrap();
        let sprite = comps.animated_sprites.get(&sprite).unwrap();
        sprite.draw(*position);
    }
}
