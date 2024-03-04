use std::collections::HashMap;

use macroquad::{prelude::*, ui::hash};

use crate::{
    game_data::GameData, input_manager::Action, map::map::Map,
    physics::collision::resolve_map_collision, sprite::indexed_sprite::IndexedSprite,
    systems::traits::SphereCollider,
};

use super::{
    animated_sprite::{AnimatedSprite, Animation},
    entities::Components,
    entity_id::Entity,
};

pub struct PlayerData {
    pub move_speed: f32,
    pub sprite_offset: Vec2,
    pub hp: u8,
    pub max_hp: u8,
}

pub fn spawn_player(
    data: &mut GameData,
    texture: Texture2D,
    entities: &mut Vec<Entity>,
    components: &mut Components,
) -> Entity {
    let id = data.new_entity();

    let indexed_sprite = IndexedSprite::new(texture, 16, vec2(8., 10.));
    let sprite = AnimatedSprite::new(
        indexed_sprite,
        HashMap::from([("idle".to_string(), Animation::new(vec![0], 0., false))]),
    );
    components.animated_sprites.insert(id, sprite);

    let collider = SphereCollider { radius: 3. };
    components.colliders.insert(id, collider);

    components.positions.insert(id, vec2(180., 120.));
    components.velocities.insert(id, Vec2::ZERO);

    let player_data = PlayerData {
        move_speed: 72.,
        sprite_offset: vec2(8., 10.),
        hp: 3,
        max_hp: 3,
    };

    components.player_data.insert(id, player_data);

    entities.push(id);
    id
}

// impl<'a> PlayerData<'a> {
//     pub fn new(texture: Texture2D, components: &mut Components) -> Self {
//         let id = Entity(hash!());

//         let indexed_sprite = IndexedSprite::new(texture, 16, vec2(8., 10.));
//         let sprite = AnimatedSprite::new(
//             indexed_sprite,
//             HashMap::from([("idle".to_string(), Animation::new(vec![0], 0., false))]),
//         );

//         let indexed_sprite = IndexedSprite::new(texture, 16, vec2(8., 10.));
//         let sprite = AnimatedSprite::new(
//             indexed_sprite,
//             HashMap::from([("idle".to_string(), Animation::new(vec![0], 0., false))]),
//         );
//         components.animated_sprites.push((id, sprite));

//         let collider = SphereCollider {
//             position: Vec2::ZERO,
//             collider_radius: 3.,
//         };
//         components.colliders.push((id, collider));

//         Self {
//             id,
//             position: vec2(180., 120.),
//             move_speed: 72.,
//             sprite_offset: vec2(8., 10.),
//             hp: 3,
//             max_hp: 3,
//         }
//     }

//     // pub fn update(&mut self, data: &GameData, map: &Map, colliders: &Vec<SphereCollider>) {
//     //     let player = self;
//     //     let mut dir = Vec2::ZERO;
//     //     if data.input.is_currently_pressed(Action::Left) {
//     //         dir += vec2(-1., 0.);
//     //     }
//     //     if data.input.is_currently_pressed(Action::Up) {
//     //         dir += vec2(0., -1.);
//     //     }
//     //     if data.input.is_currently_pressed(Action::Right) {
//     //         dir += vec2(1., 0.);
//     //     }
//     //     if data.input.is_currently_pressed(Action::Down) {
//     //         dir += vec2(0., 1.);
//     //     }
//     //     if let Some(gamepad) = data.input.gamepads.get_last_used() {
//     //         let input = vec2(gamepad.left_stick_x(), -gamepad.left_stick_y());
//     //         if input.length_squared() > 0. {
//     //             dir = input;
//     //         }
//     //     }

//     //     let mut desired_pos = if dir.length_squared() > 0. {
//     //         dir = dir.normalize();
//     //         player.position + player.move_speed * dir * get_frame_time()
//     //     } else {
//     //         player.position
//     //     };

//     //     player.position = resolve_map_collision(data, map, desired_pos, player.collider_radius);
//     // }
// }

// impl SphereCollider for PlayerData {
//     fn radius(&self) -> f32 {
//         self.collider_radius
//     }
// }

// impl Position for PlayerData {
//     fn position(&self) -> Vec2 {
//         self.position
//     }
// }

// impl Sprite for PlayerData {
//     fn texture_and_offset(&self) -> (&Texture2D, Vec2) {
//         (&self.texture, self.sprite_offset)
//     }
// }

// impl Timeable for PlayerData {
//     fn update_timers(&mut self) {}
// }
