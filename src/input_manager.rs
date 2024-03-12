use gamepads::{Button, Gamepads};
use macroquad::{
    camera::Camera2D,
    input::{
        is_key_down, is_key_pressed, is_key_released, mouse_delta_position, mouse_position, KeyCode,
    },
    math::{vec2, Vec2},
    window::screen_height,
};

use crate::entity::entities::Ecs;

pub struct InputManager {
    pub gamepads: Gamepads,
    pub last_aim_dir: Vec2,
    pub last_mouse_pos: Vec2,
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
            last_aim_dir: Vec2::X,
            last_mouse_pos: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, ecs: &Ecs, camera: &Camera2D) {
        let mut input_dir = None;

        let gamepad = self.gamepads.get_last_used();
        let mouse_pos = vec2(mouse_position().0, screen_height() - mouse_position().1);
        let mouse_delta = mouse_pos - self.last_mouse_pos;
        if mouse_delta.length_squared() > 0.001 {
            self.last_mouse_pos = mouse_pos;
            let players = ecs.check_components(|e, comps| {
                comps.player_data.contains_key(e) && comps.positions.contains_key(e)
            });

            for player_e in &players {
                let player_pos = ecs.components.positions.get(player_e).unwrap();
                let mouse_pos = camera.screen_to_world(mouse_pos);
                let delta = (mouse_pos - *player_pos).normalize();
                input_dir = Some(delta);
            }
        }

        if let Some(gamepad) = gamepad {
            let stick_input = gamepad.right_stick();
            let stick_dir = vec2(stick_input.0, -stick_input.1);
            if stick_dir.length_squared() > 0.1 {
                input_dir = Some(stick_dir.normalize());
            }
        }
        if input_dir == None {
            let mut keyboard_dir = Vec2::ZERO;
            if is_key_down(KeyCode::Left) {
                keyboard_dir -= vec2(1., 0.);
            }
            if is_key_down(KeyCode::Up) {
                keyboard_dir -= vec2(0., 1.);
            }
            if is_key_down(KeyCode::Right) {
                keyboard_dir -= vec2(-1., 0.);
            }
            if is_key_down(KeyCode::Down) {
                keyboard_dir -= vec2(0., -1.);
            }
            if keyboard_dir.length_squared() > 0.1 {
                input_dir = Some(keyboard_dir.normalize());
            }
        }

        if let Some(new_dir) = input_dir {
            self.last_aim_dir = new_dir;
        }
    }

    pub fn get_aim_dir(&mut self, camera: &Camera2D, player_pos: Vec2) -> Vec2 {
        // let mut input_dir = None;

        // let gamepad = self.gamepads.get_last_used();
        // let mouse_delta = vec2(mouse_delta_position().x, mouse_delta_position().y);
        // if mouse_delta.length_squared() > 0.1 {
        //     let mouse_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        //     let delta = (mouse_pos - player_pos).normalize();
        //     input_dir = Some(delta);
        // }

        // if let Some(gamepad) = gamepad {
        //     let stick_input = gamepad.right_stick();
        //     let stick_dir = vec2(stick_input.0, stick_input.1);
        //     if stick_dir.length_squared() > 0.1 {
        //         input_dir = Some(stick_dir.normalize());
        //     }
        // }
        // if input_dir == None {
        //     let mut keyboard_dir = Vec2::ZERO;
        //     if is_key_down(KeyCode::Left) {
        //         keyboard_dir -= vec2(-1., 0.);
        //     }
        //     if is_key_down(KeyCode::Up) {
        //         keyboard_dir -= vec2(0., -1.);
        //     }
        //     if is_key_down(KeyCode::Right) {
        //         keyboard_dir -= vec2(1., 0.);
        //     }
        //     if is_key_down(KeyCode::Down) {
        //         keyboard_dir -= vec2(0., 1.);
        //     }
        //     if keyboard_dir.length_squared() > 0.1 {
        //         input_dir = Some(keyboard_dir.normalize());
        //     }
        // }

        // if input_dir.is_none() {
        //     // TODO: return immediately
        //     input_dir = Some(self.last_aim_dir);
        // }
        // let input_dir = input_dir.unwrap();
        // self.last_aim_dir = input_dir;

        return self.last_aim_dir;

        // TODO: don't really need this?
        let dirs = [
            vec2(-1., 0.).normalize(),
            vec2(-1., -1.).normalize(),
            vec2(0., -1.).normalize(),
            vec2(1., -1.).normalize(),
            vec2(1., 0.).normalize(),
            vec2(1., 1.).normalize(),
            vec2(0., 1.).normalize(),
            vec2(-1., 1.).normalize(),
        ];

        let mut best_dir = Vec2::Y;
        let mut closest_result = 0.;
        for dir in dirs {
            let goodness = dir.dot(self.last_aim_dir);
            if goodness > closest_result {
                closest_result = goodness;
                best_dir = dir;
            }
        }
        best_dir
    }

    pub fn is_just_pressed(&self, action: Action) -> bool {
        let gamepad = self.gamepads.get_last_used();
        match action {
            Action::Left => {
                if is_key_pressed(KeyCode::A) {
                    return true;
                }
            }
            Action::Up => {
                if is_key_pressed(KeyCode::W) {
                    return true;
                }
            }
            Action::Right => {
                if is_key_pressed(KeyCode::D) {
                    return true;
                }
            }
            Action::Down => {
                if is_key_pressed(KeyCode::S) {
                    return true;
                }
            }
            Action::Confirm => {
                if is_key_pressed(KeyCode::E)
                    || is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Space)
                {
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
                if is_key_down(KeyCode::A) {
                    return true;
                }
            }
            Action::Up => {
                if is_key_down(KeyCode::W) {
                    return true;
                }
            }
            Action::Right => {
                if is_key_down(KeyCode::D) {
                    return true;
                }
            }
            Action::Down => {
                if is_key_down(KeyCode::S) {
                    return true;
                }
            }
            Action::Confirm => {
                if is_key_down(KeyCode::E)
                    || is_key_down(KeyCode::Enter)
                    || is_key_down(KeyCode::Space)
                {
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
