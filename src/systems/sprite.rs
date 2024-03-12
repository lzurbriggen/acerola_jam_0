use macroquad::{
    material::{gl_use_default_material, gl_use_material},
    math::vec2,
};
use std::cmp::Ordering;

use crate::{
    entity::entities::Ecs,
    game_data::{GameData, GameMaterial},
};

pub fn update_animated_sprites(ecs: &mut Ecs) {
    let sprites = ecs.check_components(|e, comps| comps.animated_sprites.contains_key(e));

    for sprite_e in &sprites {
        let sprite = ecs.components.animated_sprites.get_mut(&sprite_e).unwrap();
        sprite.update();
        if sprite.current_animation().1.completed
            && ecs.components.despawn_on_anim_end.contains_key(sprite_e)
        {
            ecs.despawn(*sprite_e);
        }
    }
}

pub fn draw_animated_sprites(ecs: &Ecs, data: &GameData) {
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

    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.positions.contains_key(e)
    });
    let player_pos = {
        let mut pos = vec2(360. / 2., 240. / 2.);
        for player_e in &players {
            pos = *ecs.components.positions.get(player_e).unwrap();
            break;
        }
        pos
    };

    for sprite_e in &sprites {
        let position = ecs.components.positions.get(&sprite_e).unwrap();
        let sprite = ecs.components.animated_sprites.get(&sprite_e).unwrap();
        let material = ecs.components.materials.get(&sprite_e);

        if !sprite.visible {
            continue;
        }

        let flipped =
            ecs.components.flip_to_player.get(&sprite_e).is_some() && position.x > player_pos.x;

        if let Some(material) = material {
            //     let mat = data.graphics.materials.get(mat_name).unwrap();
            //     match mat {
            //         GameMaterial::Aberration(mat) => {
            //             gl_use_material(&mat);
            //         }
            //         GameMaterial::Color(mat) => {
            //             mat.set_uniform("color", sprite.color);
            //             gl_use_material(&mat);
            //         }
            //     }
            gl_use_material(material);
        } else {
            gl_use_default_material();
        }

        sprite.draw(data, *position, flipped);
        gl_use_default_material();
    }
}
