use macroquad::prelude::*;

use crate::{
    entity::{entities::Ecs, entity_id::Entity, spawner::spawn_spawner},
    game_state::GameState,
    input_manager::InputManager,
    items::weapon::Weapon,
    map::map::Map,
    room::Room,
    settings::GameSettings,
    sprite::indexed_sprite::IndexedSprite,
    timer::Timer,
    ui::{screen_dimmer::ScreenDimmer, ui_data::UIData},
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
    pub show_fps: bool,
    pub weapon: Weapon,
    pub current_room: Room,
    pub maps: Vec<Map>,
    pub screen_dimmer: ScreenDimmer,
    pub map_change_requested: bool,
    pub paused: bool,
    pub pause_timer: Timer,
    pub show_pause_menu: bool,
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

    pub fn current_map(&self) -> &Map {
        &self.maps[self.current_room.map_index]
    }

    pub fn spawn_map_entities(&mut self, ecs: &mut Ecs) -> Vec2 {
        let mut player_pos = Vec2::ZERO;
        let mut spawner_positions = vec![];
        for (_, layer) in &self.current_map().tiled_map.layers {
            for object in &layer.objects {
                let object_pos = vec2(object.world_x + 4., object.world_y - 4.);
                // if let Some(_door_dir) = object.properties.get("door") {
                //     spawn_door(self, object_pos, ecs);
                // }
                if let Some(_door_dir) = object.properties.get("player") {
                    player_pos = object_pos;
                }
                if let Some(_) = object.properties.get("spawn") {
                    spawner_positions.push(object_pos);
                }
            }
        }

        for pos in spawner_positions {
            spawn_spawner(self, pos, ecs);
        }

        player_pos
    }
}
