use macroquad::prelude::*;

use crate::{
    game_state::GameState, input_manager::InputManager, settings::GameSettings,
    sprite::indexed_sprite::IndexedSprite, ui::ui_data::UIData,
};

pub struct Sprites {
    pub hud_heart: IndexedSprite,
}

pub struct GameData {
    pub state: GameState,
    pub settings: GameSettings,
    pub ui: UIData,
    pub sprites: Sprites,
    pub input: InputManager,
    pub camera: Camera2D,
}

impl GameData {
    pub fn update(&mut self) {
        self.input.gamepads.poll();
        self.update_camera();
    }

    pub fn update_camera(&mut self) {
        let target_size = self.settings.resolution;
        let target = vec2(target_size.x / 2., target_size.y / 2.);

        self.camera.target = target;
        self.camera.zoom = vec2(1. / target_size.x * 2., 1. / target_size.y * 2.);
        self.camera.offset = Vec2::ZERO;
    }
}
