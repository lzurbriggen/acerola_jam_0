use macroquad::prelude::*;

use crate::{
    entity::entity_id::Entity, game_state::GameState, input_manager::InputManager,
    items::weapon::Weapon, settings::GameSettings, sprite::indexed_sprite::IndexedSprite,
    ui::ui_data::UIData,
};

pub struct Sprites {
    pub hud_heart: IndexedSprite,
    pub aberration_meter: IndexedSprite,
    pub aberration_material: Material,
}

pub struct GameData {
    pub entity_index: u64,
    pub state: GameState,
    pub settings: GameSettings,
    pub ui: UIData,
    pub sprites: Sprites,
    pub input: InputManager,
    pub camera: Camera2D,
    pub debug_collisions: bool,
    pub weapon: Weapon,
}

impl GameData {
    pub fn new_entity(&mut self) -> Entity {
        self.entity_index += 1;
        Entity(self.entity_index)
    }

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
