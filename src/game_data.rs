use gamepads::Gamepads;

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
}

impl GameData {
    pub fn update(&mut self) {
        self.input.gamepads.poll();
    }
}
