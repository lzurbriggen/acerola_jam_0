use std::cmp::Ordering;

use crate::entity::entities::Ecs;

pub fn update_animated_sprites(ecs: &mut Ecs) {
    let sprites = ecs.check_components(|e, comps| comps.animated_sprites.contains_key(e));

    for sprite in &sprites {
        let sprite = ecs.components.animated_sprites.get_mut(&sprite).unwrap();
        sprite.update();
    }
}

pub fn draw_animated_sprites(ecs: &Ecs) {
    let mut sprites = ecs
        .check_components(|e, comps| {
            comps.positions.contains_key(e) && comps.animated_sprites.contains_key(e)
        })
        .clone();

    sprites.sort_by(|a, b| {
        let a_pos = ecs.components.positions.get(&a).unwrap();
        let b_pos = ecs.components.positions.get(&b).unwrap();
        if a_pos.y < b_pos.y {
            Ordering::Less
        } else if a_pos.y > b_pos.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for sprite in &sprites {
        let position = ecs.components.positions.get(&sprite).unwrap();
        let sprite = ecs.components.animated_sprites.get(&sprite).unwrap();
        sprite.draw(*position);
    }
}
