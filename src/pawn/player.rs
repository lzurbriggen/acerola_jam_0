use macroquad::prelude::*;

use crate::game_data::GameData;

pub struct Player {
    texture: Texture2D,
    position: Vec2,
    move_speed: f32,
    // relative to the texture size
    collider: Rect,
    pub hp: u8,
    pub max_hp: u8,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            position: vec2(172., 102.),
            move_speed: 72.,
            collider: Rect::new(6., 6., 4., 5.),
            hp: 3,
            max_hp: 3,
        }
    }
}

impl Player {
    pub fn update(&mut self, data: &mut GameData) {
        // let gamepads = data.gamepad.unwrap().;

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
        if dir.length_squared() > 0. {
            dir = dir.normalize();
            self.position = self.position + self.move_speed * dir * get_frame_time();
        }
    }

    pub fn draw(&self, data: &mut GameData) {
        // let scale_factor = vec2(screen_width() / 360., screen_height() / 240.);
        // let position = self
        //     .position
        //     .mul(scale_factor)
        //     .round()
        //     .div(scale_factor)
        //     .round();
        // let position = self.position.clone().round();
        let position = self.position;

        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        // draw_rectangle_ex(
        //     position.x + self.collider.x,
        //     position.y + self.collider.y,
        //     self.collider.w,
        //     self.collider.h,
        //     DrawRectangleParams {
        //         color: Color::from_rgba(0, 255, 0, 100),
        //         ..Default::default()
        //     },
        // );
    }
}
