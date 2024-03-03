use macroquad::prelude::*;

use crate::{
    game_data::GameData,
    map::map::Map,
    physics::collision::{resolve_collision, CircleCollider},
};

pub struct Player {
    texture: Texture2D,
    pub position: Vec2,
    move_speed: f32,
    collider: CircleCollider,
    pub hp: u8,
    pub max_hp: u8,
}

impl Player {
    pub fn new(texture: Texture2D, data: &GameData) -> Self {
        Self {
            texture,
            position: vec2(180., 120.),
            move_speed: 72.,
            collider: CircleCollider::new(vec2(8., 10.), 3.),
            hp: 3,
            max_hp: 3,
        }
    }
}

impl Player {
    pub fn update(&mut self, data: &mut GameData, map: &Map) {
        let mut dir = Vec2::ZERO;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dir += vec2(-1., 0.);
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dir += vec2(0., -1.);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dir += vec2(1., 0.);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dir += vec2(0., 1.);
        }
        if let Some(gamepad) = data.input.gamepads.get_last_used() {
            let input = vec2(gamepad.left_stick_x(), -gamepad.left_stick_y());
            if input.length_squared() > 0. {
                dir = input;
            }
        }

        let mut desired_pos = if dir.length_squared() > 0. {
            dir = dir.normalize();
            self.position + self.move_speed * dir * get_frame_time()
        } else {
            self.position
        };

        desired_pos = resolve_collision(data, map, desired_pos, self.collider.radius);

        self.position = desired_pos;
    }

    pub fn draw(&self, data: &mut GameData) {
        let position = self.position - self.collider.offset;
        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        if data.debug_collisions {
            draw_circle_lines(
                self.position.x,
                self.position.y,
                self.collider.radius,
                1.,
                GREEN,
            )
        }
    }
}
