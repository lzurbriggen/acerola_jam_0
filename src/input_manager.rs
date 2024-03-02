use gamepads::{Button, Gamepads};
use macroquad::input::{is_key_down, is_key_pressed, is_key_released, KeyCode};

use crate::game_data::GameData;

pub struct InputManager {
    pub gamepads: Gamepads,
}

pub enum Action {
    Left,
    Up,
    Right,
    Down,
    Confirm,
    Pause,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            gamepads: Gamepads::new(),
        }
    }

    pub fn is_just_pressed(&self, action: Action) -> bool {
        let gamepad = self.gamepads.get_last_used();
        match action {
            Action::Left => {
                if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
                    return true;
                }
            }
            Action::Up => {
                if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                    return true;
                }
            }
            Action::Right => {
                if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
                    return true;
                }
            }
            Action::Down => {
                if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                    return true;
                }
            }
            Action::Confirm => {
                if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
                    return true;
                }
            }
            Action::Pause => {
                if is_key_pressed(KeyCode::Escape) {
                    return true;
                }
            }
        }

        if let Some(gamepad) = gamepad {
            match action {
                Action::Left => {
                    if gamepad.is_just_pressed(Button::DPadLeft) {
                        return true;
                    }
                }
                Action::Up => {
                    if gamepad.is_just_pressed(Button::DPadUp) {
                        return true;
                    }
                }
                Action::Right => {
                    if gamepad.is_just_pressed(Button::DPadRight) {
                        return true;
                    }
                }
                Action::Down => {
                    if gamepad.is_just_pressed(Button::DPadDown) {
                        return true;
                    }
                }
                Action::Confirm => {
                    if gamepad.is_just_pressed(Button::ActionDown) {
                        return true;
                    }
                }
                Action::Pause => {
                    if gamepad.is_just_pressed(Button::RightCenterCluster) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_currently_pressed(&self, action: Action) -> bool {
        let gamepad = self.gamepads.get_last_used();
        match action {
            Action::Left => {
                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    return true;
                }
            }
            Action::Up => {
                if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                    return true;
                }
            }
            Action::Right => {
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    return true;
                }
            }
            Action::Down => {
                if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                    return true;
                }
            }
            Action::Confirm => {
                if is_key_down(KeyCode::E) || is_key_down(KeyCode::Enter) {
                    return true;
                }
            }
            Action::Pause => {
                if is_key_down(KeyCode::Escape) {
                    return true;
                }
            }
        }

        if let Some(gamepad) = gamepad {
            match action {
                Action::Left => {
                    if gamepad.is_currently_pressed(Button::DPadLeft) {
                        return true;
                    }
                }
                Action::Up => {
                    if gamepad.is_currently_pressed(Button::DPadUp) {
                        return true;
                    }
                }
                Action::Right => {
                    if gamepad.is_currently_pressed(Button::DPadRight) {
                        return true;
                    }
                }
                Action::Down => {
                    if gamepad.is_currently_pressed(Button::DPadDown) {
                        return true;
                    }
                }
                Action::Confirm => {
                    if gamepad.is_currently_pressed(Button::ActionDown) {
                        return true;
                    }
                }
                Action::Pause => {
                    if gamepad.is_currently_pressed(Button::RightCenterCluster) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_just_released(&self, action: Action) -> bool {
        let gamepad = self.gamepads.get_last_used();
        match action {
            Action::Left => {
                if is_key_released(KeyCode::A) || is_key_released(KeyCode::Left) {
                    return true;
                }
            }
            Action::Up => {
                if is_key_released(KeyCode::W) || is_key_released(KeyCode::Up) {
                    return true;
                }
            }
            Action::Right => {
                if is_key_released(KeyCode::D) || is_key_released(KeyCode::Right) {
                    return true;
                }
            }
            Action::Down => {
                if is_key_released(KeyCode::S) || is_key_released(KeyCode::Down) {
                    return true;
                }
            }
            Action::Confirm => {
                if is_key_released(KeyCode::E) || is_key_released(KeyCode::Enter) {
                    return true;
                }
            }
            Action::Pause => {
                if is_key_released(KeyCode::Escape) {
                    return true;
                }
            }
        }

        if let Some(gamepad) = gamepad {
            match action {
                Action::Left => {
                    if gamepad.is_just_released(Button::DPadLeft) {
                        return true;
                    }
                }
                Action::Up => {
                    if gamepad.is_just_released(Button::DPadUp) {
                        return true;
                    }
                }
                Action::Right => {
                    if gamepad.is_just_released(Button::DPadRight) {
                        return true;
                    }
                }
                Action::Down => {
                    if gamepad.is_just_released(Button::DPadDown) {
                        return true;
                    }
                }
                Action::Confirm => {
                    if gamepad.is_just_released(Button::ActionDown) {
                        return true;
                    }
                }
                Action::Pause => {
                    if gamepad.is_just_released(Button::RightCenterCluster) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
