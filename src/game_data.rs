use std::ops::Range;

use gamepads::{Gamepad, Gamepads};

use crate::{game_state::GameState, settings::GameSettings};

pub struct GameData {
    pub state: GameState,
    pub gamepads: Gamepads,
    pub settings: GameSettings,
    pub gamepad: Option<Gamepad>,
}

const DEADZONE: f32 = 0.1;
const DEADZONE_RANGE: Range<f32> = -DEADZONE..DEADZONE;

impl GameData {
    pub fn update(&mut self) {
        self.gamepads.poll();
        // primitive way to get the last used gamepad
        'outer: for gamepad in self.gamepads.all() {
            if DEADZONE_RANGE.contains(&gamepad.left_stick_x())
                || DEADZONE_RANGE.contains(&gamepad.left_stick_y())
                || DEADZONE_RANGE.contains(&gamepad.right_stick_x())
                || DEADZONE_RANGE.contains(&gamepad.right_stick_y())
            {
                self.gamepad = Some(gamepad);
                break 'outer;
            }
            for _ in gamepad.all_currently_pressed() {
                self.gamepad = Some(gamepad);
                break 'outer;
            }
            for _ in gamepad.all_just_pressed() {
                self.gamepad = Some(gamepad);
                break 'outer;
            }
        }
    }
}
