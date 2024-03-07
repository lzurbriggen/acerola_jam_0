use macroquad::material::{gl_use_default_material, gl_use_material};
use std::cmp::Ordering;

use crate::entity::entities::Ecs;

pub fn update_animated_sprites(ecs: &mut Ecs) {
    let sprites = ecs.check_components(|e, comps| comps.animated_sprites.contains_key(e));

    for sprite_e in &sprites {
        let sprite = ecs.components.animated_sprites.get_mut(&sprite_e).unwrap();
        let anim_completed = sprite.update();
        if anim_completed && ecs.components.despawn_on_anim_end.contains_key(sprite_e) {
            ecs.despawn(*sprite_e);
        }
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
    sprites.sort_by(|a, b| {
        let a_offset = ecs.components.layer_offset.get(&a).unwrap_or(&0);
        let b_offset = ecs.components.layer_offset.get(&b).unwrap_or(&0);
        if a_offset < b_offset {
            Ordering::Less
        } else if a_offset > b_offset {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for sprite_e in &sprites {
        let position = ecs.components.positions.get(&sprite_e).unwrap();
        let sprite = ecs.components.animated_sprites.get(&sprite_e).unwrap();
        let material = ecs.components.materials.get(&sprite_e);

        if let Some(material) = material {
            gl_use_material(&material);
        }

        sprite.draw(*position);

        gl_use_default_material();
    }
}
