use std::collections::HashMap;

use macroquad::{audio::Sound, prelude::*};

use crate::{
    entity::{entities::Ecs, entity_id::Entity, player::spawn_player, spawner::spawn_spawner},
    game_state::GameState,
    input_manager::InputManager,
    items::weapon::{Launcher, Weapon},
    map::map::Map,
    room::Room,
    settings::GameSettings,
    timer::Timer,
    ui::{death_screen::DeathScreen, screen_dimmer::ScreenDimmer, ui_data::UIData},
};

pub enum GameMaterial {
    Aberration(Material),
    Color(Material),
}

pub struct Graphics {
    pub aberration_meter_material: Material,
    pub aberration_material: Material,
    pub noise1_texture: Texture2D,
    pub noise2_texture: Texture2D,
    pub materials: HashMap<String, GameMaterial>,
    pub textures: HashMap<&'static str, Texture2D>,
}

pub struct Audio {
    pub ui_switch: Sound,
    pub shoot: Sound,
    pub death: Sound,
    pub death2: Sound,
    pub spawn: Sound,
    pub kill: Sound,
    pub confirm: Sound,
    pub confirm2: Sound,
    pub hit: Sound,
    pub hit2: Sound,
}

pub struct GameData {
    pub entity_index: u64,
    pub state: GameState,
    pub settings: GameSettings,
    pub ui: UIData,
    pub graphics: Graphics,
    pub audio: Audio,
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
    pub death_screen: DeathScreen,
    pub dead: bool,
}

impl GameData {
    pub fn new(
        initial_entity_index: u64,
        settings: GameSettings,
        ui_data: UIData,
        maps: Vec<Map>,
        graphics: Graphics,
        audio: Audio,
        death_texture: Texture2D,
    ) -> Self {
        Self {
            entity_index: initial_entity_index,
            settings,
            state: GameState::default(),
            ui: ui_data,
            graphics,
            audio,
            input: InputManager::new(),
            camera: Camera2D::default(),
            debug_collisions: false,
            #[cfg(debug_assertions)]
            show_fps: true,
            #[cfg(not(debug_assertions))]
            show_fps: false,
            weapon: Weapon::Launcher(Launcher::new()),
            current_room: Room::new(maps.len(), 3.),
            maps,
            screen_dimmer: ScreenDimmer::new(),
            map_change_requested: false,
            paused: false,
            pause_timer: Timer::new(1., false),
            show_pause_menu: false,
            death_screen: DeathScreen::new(death_texture),
            dead: true,
        }
    }

    pub fn reset(&mut self) {
        self.state = GameState::Intro;
        self.weapon = Weapon::Launcher(Launcher::new());
        self.current_room = Room::new(self.maps.len(), 3.);
        self.dead = false;
    }

    pub fn new_entity(&mut self) -> Entity {
        self.entity_index += 1;
        Entity(self.entity_index)
    }

    pub fn update(&mut self) {
        self.pause_timer.update();
        self.input.gamepads.poll();
        self.update_camera();

        for mat in &self.graphics.materials {
            match mat.1 {
                GameMaterial::Aberration(mat) => {
                    // mat.set_uniform("intensity", 2.58f32);
                    mat.set_uniform("time", get_time() as f32);
                }
                GameMaterial::Color(_mat) => {}
            }
        }
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

pub fn reset_game(data: &mut GameData, ecs: &mut Ecs) {
    data.reset();
    let entities = ecs.check_components(|_, _| true);
    for entity in entities {
        let entity_i = ecs.entities.iter().position(|e| e == &entity).unwrap();
        ecs.entities.remove(entity_i);
        ecs.remove_all_components(&entity);
    }
    data.current_room = Room::new(data.maps.len(), rand::gen_range(1., 20.));
    spawn_player(data, ecs);
    let new_player_pos = data.spawn_map_entities(ecs);
    data.current_room.started = true;
    let players = ecs.check_components(|e, comps| {
        comps.player_data.contains_key(e) && comps.positions.contains_key(e)
    });
    for player_e in &players {
        let pos = ecs.components.positions.get_mut(player_e).unwrap();
        *pos = new_player_pos;
    }
}
