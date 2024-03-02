use gamepads::Gamepads;

use crate::{game_state::GameState, settings::GameSettings, ui::ui_data::UIData};

pub struct GameData {
    pub state: GameState,
    pub gamepads: Gamepads,
    pub settings: GameSettings,
    pub ui: UIData,
}

impl GameData {
    pub fn update(&mut self) {
        self.gamepads.poll();
    }
}
