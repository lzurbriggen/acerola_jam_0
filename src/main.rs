use entity::entities::Entities;
use fps_counter::FPSCounter;
use game_state::GameState;
use macroquad::{audio, miniquad::window::set_mouse_cursor, prelude::*};
use macroquad_tiled::load_map;
use settings::{GameSettings, WindowSize};
use systems::{
    collision::draw_colliders,
    door::handle_door_collisions,
    enemy::update_enemies,
    player::update_player,
    spawn::spawn_creatures,
    sprite::{draw_multi_sprites, draw_simple_sprites},
    timer::update_timers,
};
use ui::{pause_menu::pause_menu, ui_data::UIData};

use crate::{
    entity::player::Player,
    game_data::{GameData, Sprites},
    input_manager::{Action, InputManager},
    map::map::Map,
    sprite::indexed_sprite::IndexedSprite,
    ui::hud::draw_hp,
};

mod entity;
mod fps_counter;
mod game_data;
mod game_state;
mod input_manager;
mod map;
mod physics;
mod room;
mod settings;
mod sprite;
mod systems;
mod timer;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Acrola Jam 0".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1440,
        window_height: 960,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            framebuffer_alpha: false,
            swap_interval: None,
            ..Default::default()
        },
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let render_target = render_target(360, 240);
    render_target.texture.set_filter(FilterMode::Nearest);

    let mut font = load_ttf_font("fonts/Bitfantasy.ttf").await.unwrap();
    font.set_filter(FilterMode::Nearest);

    let mut icon_font = load_ttf_font("fonts/Zicons.ttf").await.unwrap();
    icon_font.set_filter(FilterMode::Nearest);

    // UI assets
    let button_texture: Texture2D = load_texture("ui/button_bg.png").await.unwrap();
    button_texture.set_filter(FilterMode::Nearest);
    let button_texture_hover: Texture2D = load_texture("ui/button_bg_hover.png").await.unwrap();
    button_texture_hover.set_filter(FilterMode::Nearest);
    let button_texture_pressed: Texture2D = load_texture("ui/button_bg_clicked.png").await.unwrap();
    button_texture_pressed.set_filter(FilterMode::Nearest);
    let frame_texture: Texture2D = load_texture("ui/window_bg.png").await.unwrap();
    frame_texture.set_filter(FilterMode::Nearest);
    let focus_bg_texture: Texture2D = load_texture("ui/focus_bg.png").await.unwrap();
    focus_bg_texture.set_filter(FilterMode::Nearest);

    // Sfx
    let button_click_sfx = audio::load_sound("audio/ui/bookClose.ogg").await.unwrap();

    let ui_data = UIData {
        button_texture: button_texture,
        button_texture_hover: button_texture_hover,
        button_texture_pressed: button_texture_pressed,
        button_click_sfx: button_click_sfx,
        frame_texture: frame_texture.clone(),
        focus_background_texture: focus_bg_texture,
        font: font.clone(),
        icon_font: icon_font.clone(),
        text_color: Color::from_hex(0xe4d2aa),
        text_shadow_color: Color::from_hex(0xb4202a),
        focus: None,
    };

    let mut fullscreen = false;

    let camera = Camera2D::default();

    let mut fps_counter = FPSCounter::default();

    let mut paused = false;

    let hud_heart_texture: Texture2D = load_texture("ui/heart_01.png").await.unwrap();
    hud_heart_texture.set_filter(FilterMode::Nearest);
    let hopper_texture: Texture2D = load_texture("entities/hopper_01.png").await.unwrap();
    hopper_texture.set_filter(FilterMode::Nearest);

    let sprites = Sprites {
        hud_heart: IndexedSprite::new(hud_heart_texture, 16, Vec2::ZERO),
        hopper: IndexedSprite::new(hopper_texture, 16, vec2(-8., -8.)),
    };

    let settings = GameSettings::default();

    // Map
    let tileset = load_texture("map/tileset_01.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);
    let tiled_map_json = load_string("map/example_01.tmj").await.unwrap();
    let tiled_map = load_map(tiled_map_json.as_str(), &[("tileset_01.png", tileset)], &[]).unwrap();
    let map = Map::new(tiled_map, settings.resolution);

    let mut data = GameData {
        settings,
        state: GameState::default(),
        ui: ui_data,
        sprites,
        input: InputManager::new(),
        camera,
        debug_collisions: false,
    };
    data.settings.set_window_size(WindowSize::W1440);

    let player_texture: Texture2D = load_texture("entities/player_01.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let player = Player::new(player_texture, &data);

    let mut entities = Entities {
        player,
        doors: vec![],
        spawners: vec![],
        enemies: vec![],
    };

    let (map_doors, map_spawners) = map.get_entities();
    entities.doors = [entities.doors, map_doors].concat();
    entities.spawners = [entities.spawners, map_spawners].concat();

    loop {
        data.update();
        set_mouse_cursor(miniquad::CursorIcon::Default);

        set_camera(&data.camera);

        clear_background(BLACK);

        if (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
            && is_key_pressed(KeyCode::Enter)
        {
            fullscreen = !fullscreen;

            if fullscreen {
                data.settings
                    .set_window_size(settings::WindowSize::Fullscreen);
            } else {
                data.settings.set_window_size(WindowSize::default());
            }
        }

        if is_key_pressed(KeyCode::F1) {
            data.debug_collisions = !data.debug_collisions;
        }

        if data.input.is_just_pressed(Action::Pause) {
            paused = !paused;
            if !paused {
                data.ui.focus = None;
            }
        }

        map.draw_base();

        spawn_creatures(&data, &mut entities);
        update_timers(&mut entities);
        update_player(&mut data, &map, &mut entities);
        update_enemies(&mut data, &map, &mut entities);
        handle_door_collisions(&data, &mut entities);

        draw_simple_sprites(&entities);
        draw_multi_sprites(&entities);
        map.draw_upper();

        if data.debug_collisions {
            draw_colliders(&data, &entities);
            map.draw_colliders();
        }

        draw_hp(&data, entities.player.hp, entities.player.max_hp);

        fps_counter.update_and_draw(&mut data);

        if paused && pause_menu(&mut data) {
            break;
        }

        next_frame().await
    }
}
