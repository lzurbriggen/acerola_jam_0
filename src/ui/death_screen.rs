use macroquad::{
    color::Color,
    math::{vec2, Rect, Vec2},
    shapes::{draw_rectangle_ex, DrawRectangleParams},
    texture::{draw_texture, Texture2D},
};

use crate::{game_data::GameData, timer::Timer};

use super::button::button;

pub struct DeathScreen {
    texture: Texture2D,
    start_timer: Timer,
    timer: Timer,
    text_timer: Timer,
    show_button_timer: Timer,
}

impl DeathScreen {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            start_timer: Timer::new(0.7, false),
            timer: Timer::new(1.5, false),
            text_timer: Timer::new(0.3, false),
            show_button_timer: Timer::new(2.5, false),
        }
    }

    pub fn update(&mut self) {
        self.start_timer.update();
        self.timer.update();
        self.text_timer.update();
        self.show_button_timer.update();

        if self.start_timer.just_completed() {
            self.timer.reset();
            self.text_timer.reset();
            self.show_button_timer.reset();
        }
    }

    pub fn show(&mut self) {
        self.start_timer.reset();
    }

    pub fn draw(&self, data: &GameData) -> bool {
        if !self.start_timer.completed() {
            return false;
        }
        draw_rectangle_ex(
            0.,
            0.,
            360.,
            240.,
            DrawRectangleParams {
                color: Color::from_rgba(0, 0, 0, ((1. - self.timer.progress()) * 220.) as u8),
                ..Default::default()
            },
        );
        draw_texture(
            &self.texture,
            0.,
            0.,
            Color::from_rgba(
                255,
                255,
                255,
                ((1. - self.text_timer.progress()) * 255.) as u8,
            ),
        );

        let mut should_restart = false;

        if self.show_button_timer.completed() {
            let center = vec2(360. / 2., 240. / 2.);
            let button_width = 90.;
            if button(
                &data,
                &Rect::new(center.x - button_width / 2., 180., button_width, 20.),
                true,
                "Try Again",
                None,
                Vec2::ZERO,
            ) {
                should_restart = true;
            }
        }

        should_restart
    }
}
