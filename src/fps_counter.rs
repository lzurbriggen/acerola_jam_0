use macroquad::prelude::*;

use crate::ui::ui_data::UIData;

pub struct FPSCounter {
    smoothing_factor: f32,
    refresh_freq: f32,
    time_since_last_update: f32,
    average_fps: f32,
    fps: f32,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self {
            smoothing_factor: 0.9,
            refresh_freq: 0.1,
            time_since_last_update: 0.,
            average_fps: 0.,
            fps: 0.,
        }
    }
}

impl FPSCounter {
    pub fn update_and_draw(&mut self, ui_data: &UIData) {
        self.average_fps = self.smoothing_factor * self.average_fps
            + (1. - self.smoothing_factor) * 1. / get_frame_time();

        draw_text_ex(
            format!("FPS: {}", self.fps.round()).as_str(),
            2.,
            10.,
            TextParams {
                font: Some(&ui_data.font),
                font_size: 16,
                ..Default::default()
            },
        );

        if self.time_since_last_update <= self.refresh_freq {
            self.time_since_last_update += get_frame_time();
            return;
        }

        self.fps = self.average_fps;

        self.time_since_last_update = 0.;
    }
}
