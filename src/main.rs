use std::collections::HashMap;

use entity::{
    entities::Ecs,
    entity_id::Entity,
    events::{DamageEvent, DeathEvent},
    upgrades::{Upgrade, Upgrades, WeaponUpgrade},
};
use fps_counter::FPSCounter;
use game_data::{reset_game, Audio, GameMaterial};
use game_state::GameState;
use items::weapon::{Balls, Dash, Launcher, Weapon, WeaponType};
use macroquad::{
    audio::{self, play_sound, set_sound_volume},
    miniquad::window::set_mouse_cursor,
    prelude::*,
};
use macroquad_tiled::load_map;
use room::Room;
use settings::{GameSettings, WindowSize};
use sprite::{
    aberration_material::create_aberration_material, flash_material::create_sprite_color_material,
};
use systems::{
    collision::draw_colliders,
    damageable::{
        apply_damage, damage_on_collision, despawn_on_collision, flash_on_damage, handle_death,
        kill_entities, update_damageables,
    },
    enemy::update_enemies,
    movement::move_entities,
    player::update_player,
    spawn::spawn_creatures,
    sprite::{draw_animated_sprites, update_animated_sprites},
    timer::update_timers,
    weapon::update_weapon,
};
use ui::{
    hud::{create_aberration_meter_material, AberrationMeter, HudHearts},
    icon,
    intro_screen::IntroScreen,
    pause_menu::pause_menu,
    ui_data::UIData,
    upgrade_screen::UpgradeScreen,
};

use crate::{
    game_data::{GameData, Graphics},
    input_manager::Action,
    map::map::Map,
};

mod entity;
mod fps_counter;
mod game_data;
mod game_state;
mod input_manager;
mod items;
mod map;
mod physics;
mod rand_utils;
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
        icon: Some(icon::set()),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

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
    let frame_texture_pretty: Texture2D = load_texture("ui/window_bg_pretty.png").await.unwrap();
    frame_texture_pretty.set_filter(FilterMode::Nearest);
    let focus_bg_texture: Texture2D = load_texture("ui/focus_bg.png").await.unwrap();
    focus_bg_texture.set_filter(FilterMode::Nearest);

    let ui_data = UIData {
        button_texture: button_texture,
        button_texture_hover: button_texture_hover,
        button_texture_pressed: button_texture_pressed,
        frame_texture: frame_texture.clone(),
        frame_texture_pretty: frame_texture_pretty.clone(),
        focus_background_texture: focus_bg_texture,
        font: font.clone(),
        icon_font: icon_font.clone(),
        text_color: Color::from_hex(0xe4d2aa),
        text_shadow_color: Color::from_hex(0xb4202a),
        focus: None,
    };

    let mut fullscreen = false;

    let mut fps_counter = FPSCounter::default();

    let hud_heart_texture: Texture2D = load_texture("ui/heart_01.png").await.unwrap();
    hud_heart_texture.set_filter(FilterMode::Nearest);
    let hopper_texture: Texture2D = load_texture("entities/hopper_01.png").await.unwrap();
    hopper_texture.set_filter(FilterMode::Nearest);
    let spitter_texture: Texture2D = load_texture("entities/spitter.png").await.unwrap();
    spitter_texture.set_filter(FilterMode::Nearest);
    let stomper_texture: Texture2D = load_texture("entities/stomper.png").await.unwrap();
    stomper_texture.set_filter(FilterMode::Nearest);
    let skull_texture: Texture2D = load_texture("entities/skull_01.png").await.unwrap();
    skull_texture.set_filter(FilterMode::Nearest);
    let bullet_texture: Texture2D = load_texture("entities/bullet_01.png").await.unwrap();
    bullet_texture.set_filter(FilterMode::Nearest);
    let bullet_enemy_texture: Texture2D = load_texture("entities/bullet_enemy.png").await.unwrap();
    bullet_enemy_texture.set_filter(FilterMode::Nearest);
    let dust_texture: Texture2D = load_texture("entities/dust_01.png").await.unwrap();
    dust_texture.set_filter(FilterMode::Nearest);
    let blood_texture: Texture2D = load_texture("entities/blood_01.png").await.unwrap();
    blood_texture.set_filter(FilterMode::Nearest);
    let aberration_meter_texture: Texture2D =
        load_texture("ui/aberration_meter.png").await.unwrap();
    aberration_meter_texture.set_filter(FilterMode::Nearest);
    let noise1_texture: Texture2D = load_texture("entities/Perlin_16-128x128.png")
        .await
        .unwrap();
    noise1_texture.set_filter(FilterMode::Nearest);
    let noise2_texture: Texture2D = load_texture("entities/Perlin_15-128x128.png")
        .await
        .unwrap();
    noise2_texture.set_filter(FilterMode::Nearest);
    let aberration_meter_mask_texture: Texture2D =
        load_texture("ui/aberration_meter_mask.png").await.unwrap();
    aberration_meter_mask_texture.set_filter(FilterMode::Nearest);
    let death_texture: Texture2D = load_texture("ui/death.png").await.unwrap();
    death_texture.set_filter(FilterMode::Nearest);
    let player_texture: Texture2D = load_texture("entities/player_01.png").await.unwrap();
    player_texture.set_filter(FilterMode::Nearest);
    let intro_screen_texture: Texture2D = load_texture("ui/intro_screen.png").await.unwrap();
    intro_screen_texture.set_filter(FilterMode::Nearest);
    let health_texture: Texture2D = load_texture("entities/health.png").await.unwrap();
    health_texture.set_filter(FilterMode::Nearest);
    let anomaly_big_texture: Texture2D = load_texture("entities/anomaly_big_01.png").await.unwrap();
    anomaly_big_texture.set_filter(FilterMode::Nearest);
    let anomaly_small_texture: Texture2D =
        load_texture("entities/anomaly_small_01.png").await.unwrap();
    anomaly_small_texture.set_filter(FilterMode::Nearest);

    let upgrade_frame_inner_texture: Texture2D =
        load_texture("ui/upgrade_frame_inner.png").await.unwrap();
    upgrade_frame_inner_texture.set_filter(FilterMode::Nearest);
    let upgrade_frame_inner_dark_texture: Texture2D =
        load_texture("ui/upgrade_frame_inner_dark.png")
            .await
            .unwrap();
    upgrade_frame_inner_dark_texture.set_filter(FilterMode::Nearest);
    let upgrade_banner_item_texture: Texture2D =
        load_texture("ui/upgrade_banner_item.png").await.unwrap();
    upgrade_banner_item_texture.set_filter(FilterMode::Nearest);
    let upgrade_banner_upgrade_texture: Texture2D =
        load_texture("ui/upgrade_banner_upgrade.png").await.unwrap();
    upgrade_banner_upgrade_texture.set_filter(FilterMode::Nearest);
    let upgrade_banner_weapon_texture: Texture2D =
        load_texture("ui/upgrade_banner_weapon.png").await.unwrap();
    upgrade_banner_weapon_texture.set_filter(FilterMode::Nearest);

    let upgrade_launcher_texture: Texture2D =
        load_texture("ui/upgrade_launcher.png").await.unwrap();
    upgrade_launcher_texture.set_filter(FilterMode::Nearest);
    let upgrade_balls_texture: Texture2D = load_texture("ui/upgrade_balls.png").await.unwrap();
    upgrade_balls_texture.set_filter(FilterMode::Nearest);
    let upgrade_dash_texture: Texture2D = load_texture("ui/upgrade_dash.png").await.unwrap();
    upgrade_dash_texture.set_filter(FilterMode::Nearest);
    let upgrade_common_max_hp_texture: Texture2D =
        load_texture("ui/upgrade_common_max_hp.png").await.unwrap();
    upgrade_common_max_hp_texture.set_filter(FilterMode::Nearest);
    let upgrade_item_hp_texture: Texture2D = load_texture("ui/upgrade_item_hp.png").await.unwrap();
    upgrade_item_hp_texture.set_filter(FilterMode::Nearest);
    let upgrade_item_anomaly_big_texture: Texture2D =
        load_texture("ui/upgrade_item_anomaly_big.png")
            .await
            .unwrap();
    upgrade_item_anomaly_big_texture.set_filter(FilterMode::Nearest);
    let upgrade_item_anomaly_small_texture: Texture2D =
        load_texture("ui/upgrade_item_anomaly_small.png")
            .await
            .unwrap();
    upgrade_item_anomaly_small_texture.set_filter(FilterMode::Nearest);

    let mut materials = HashMap::new();
    let aberration_material = create_aberration_material();
    aberration_material.set_texture("noise1", noise1_texture.clone());
    aberration_material.set_texture("noise2", noise2_texture.clone());
    materials.insert(
        "aberration".to_string(),
        GameMaterial::Aberration(aberration_material),
    );

    let color_material = create_sprite_color_material();
    materials.insert("color".to_string(), GameMaterial::Color(color_material));

    let textures = HashMap::from([
        ("hopper", hopper_texture),
        ("spitter", spitter_texture),
        ("stomper", stomper_texture),
        ("skull", skull_texture),
        ("bullet", bullet_texture),
        ("bullet_enemy", bullet_enemy_texture),
        ("dust", dust_texture),
        ("blood", blood_texture),
        ("player", player_texture),
        ("intro_screen", intro_screen_texture),
        ("hud_heart", hud_heart_texture),
        ("aberration_meter", aberration_meter_texture),
        ("health", health_texture),
        ("anomaly_big", anomaly_big_texture),
        ("anomaly_small", anomaly_small_texture),
        ("upgrade_frame_inner", upgrade_frame_inner_texture),
        ("upgrade_frame_inner_dark", upgrade_frame_inner_dark_texture),
        ("upgrade_banner_item", upgrade_banner_item_texture),
        ("upgrade_banner_upgrade", upgrade_banner_upgrade_texture),
        ("upgrade_banner_weapon", upgrade_banner_weapon_texture),
        ("upgrade_launcher", upgrade_launcher_texture),
        ("upgrade_balls", upgrade_balls_texture),
        ("upgrade_dash", upgrade_dash_texture),
        ("upgrade_common_max_hp", upgrade_common_max_hp_texture),
        ("upgrade_item_hp", upgrade_item_hp_texture),
        ("upgrade_item_anomaly_big", upgrade_item_anomaly_big_texture),
        (
            "upgrade_item_anomaly_small",
            upgrade_item_anomaly_small_texture,
        ),
    ]);

    let graphics = Graphics {
        aberration_meter_material: create_aberration_meter_material(),
        aberration_material: create_aberration_material(),
        noise1_texture,
        noise2_texture,
        materials,
        textures,
    };

    // Sfx
    let ui_switch_sfx = audio::load_sound("audio/mini_sounds_pack_40/touch_4.wav")
        .await
        .unwrap();
    let shoot_sfx = audio::load_sound("audio/mini_sounds_pack_40/hurt_1.wav")
        .await
        .unwrap();
    let death_sfx = audio::load_sound("audio/mini_sounds_pack_40/expl_2.wav")
        .await
        .unwrap();
    let death2_sfx = audio::load_sound("audio/mini_sounds_pack_40/down.wav")
        .await
        .unwrap();
    let spawn_sfx = audio::load_sound("audio/mini_sounds_pack_40/touch_4.wav")
        .await
        .unwrap();
    let kill_sfx = audio::load_sound("audio/mini_sounds_pack_40/touch_4.wav")
        .await
        .unwrap();
    let confirm_sfx = audio::load_sound("audio/ui/bookClose.ogg").await.unwrap();
    let confirm2_sfx = audio::load_sound("audio/mini_sounds_pack_40/jump_5.wav")
        .await
        .unwrap();
    let hit_sfx = audio::load_sound("audio/mini_sounds_pack_40/hurt_5.wav")
        .await
        .unwrap();
    let hit2_sfx = audio::load_sound("audio/mini_sounds_pack_40/2.wav")
        .await
        .unwrap();

    // Music
    let music1 = audio::load_sound("audio/music/game_240308_mirituhg_battle 2024-03-12 1521.wav")
        .await
        .unwrap();

    let audio = Audio {
        ui_switch: ui_switch_sfx.clone(),
        shoot: shoot_sfx.clone(),
        death: death_sfx.clone(),
        death2: death2_sfx.clone(),
        spawn: spawn_sfx.clone(),
        kill: kill_sfx.clone(),
        confirm: confirm_sfx.clone(),
        confirm2: confirm2_sfx.clone(),
        hit: hit_sfx.clone(),
        hit2: hit2_sfx.clone(),
        music1: music1,
    };

    let settings = GameSettings::default();

    let mut entity_index = 0;

    // Map
    let tileset = load_texture("map/tileset_01.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map1_json = load_string("map/example_01.tmj").await.unwrap();
    let tiled_map1 = load_map(
        tiled_map1_json.as_str(),
        &[("tileset_01.png", tileset.clone())],
        &[],
    )
    .unwrap();
    entity_index += 1;
    let map1 = Map::new(Entity(entity_index), &settings, tiled_map1);

    let tiled_map2_json = load_string("map/map2.tmj").await.unwrap();
    let tiled_map2 = load_map(
        tiled_map2_json.as_str(),
        &[("tileset_01.png", tileset.clone())],
        &[],
    )
    .unwrap();
    entity_index += 1;
    let map2 = Map::new(Entity(entity_index), &settings, tiled_map2);

    let tiled_map3_json = load_string("map/map3.tmj").await.unwrap();
    let tiled_map3 = load_map(
        tiled_map3_json.as_str(),
        &[("tileset_01.png", tileset.clone())],
        &[],
    )
    .unwrap();
    entity_index += 1;
    let map3 = Map::new(Entity(entity_index), &settings, tiled_map3);

    let maps = vec![map1, map2, map3];

    let mut data = GameData::new(
        entity_index,
        settings,
        ui_data,
        maps,
        graphics,
        audio,
        death_texture,
    );
    data.reset();
    data.settings.set_window_size(WindowSize::W1440);

    let mut ecs = Ecs::default();

    let mut collisions = HashMap::new();
    let mut damage_events = Vec::<DamageEvent>::new();
    let mut death_events = Vec::<DeathEvent>::new();

    let hud_hearts = HudHearts::new(&data);
    let aberration_meter = AberrationMeter::new(&data);

    data.graphics
        .aberration_meter_material
        .set_texture("noise1", data.graphics.noise1_texture.clone());
    data.graphics
        .aberration_meter_material
        .set_texture("noise2", data.graphics.noise2_texture.clone());
    data.graphics
        .aberration_meter_material
        .set_texture("mask", aberration_meter_mask_texture.clone());

    let mut intro_screen = IntroScreen::new(&data);

    let mut upgrade_screen = UpgradeScreen::new(Upgrades::weapon_selection());

    play_sound(
        &data.audio.music1,
        audio::PlaySoundParams {
            looped: true,
            volume: data.settings.music_volume,
        },
    );

    let post_processing_material = create_aberration_material();

    let mut camera_target = render_target(screen_width() as u32, screen_height() as u32);
    camera_target.texture.set_filter(FilterMode::Nearest);
    data.camera.render_target = Some(camera_target);
    loop {
        set_camera(&data.camera);

        let screen_size = (screen_width(), screen_height());
        if data.previous_window_size != screen_size {
            data.previous_window_size = screen_size;
            camera_target = render_target(screen_size.0 as u32, screen_size.1 as u32);
            camera_target.texture.set_filter(FilterMode::Nearest);
            data.camera.render_target = Some(camera_target);
        }

        data.update();
        set_sound_volume(&data.audio.music1, data.settings.music_volume);

        data.current_room.check_completed(&ecs);
        if data.state == GameState::Playing {
            if is_key_pressed(KeyCode::F5) {
                reset_game(&mut data, &mut ecs);
            }

            if is_key_pressed(KeyCode::F3) {
                upgrade_screen.visible = true;
            }

            let despawned_entities = &ecs.marked_for_despawn.clone();
            for entity in despawned_entities {
                let entity_i = ecs.entities.iter().position(|e| e == entity);
                if let Some(index) = entity_i {
                    ecs.entities.remove(index);
                    ecs.remove_all_components(entity);
                }
            }
            if !data.paused {
                ecs.marked_for_despawn.clear();
                death_events.clear();
            }

            // data.graphics
            //     .aberration_meter_material
            //     .set_uniform("intensity", 1.2f32);
            // data.graphics
            //     .aberration_meter_material
            //     .set_uniform("time", get_time() as f32);
        }

        post_processing_material.set_texture("noise1", data.graphics.noise1_texture.clone());
        post_processing_material.set_texture("noise2", data.graphics.noise2_texture.clone());

        set_mouse_cursor(miniquad::CursorIcon::Default);

        data.input.update(&ecs, &data.camera);
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
            if data.paused {
                data.paused = false;
                data.show_pause_menu = false;
            } else {
                data.paused = true;
                data.show_pause_menu = true;
            }
        }

        if data.state == GameState::Playing {
            // Map transition
            if is_key_pressed(KeyCode::F6) {
                data.map_change_requested = true;
                data.screen_dimmer.dim();
                data.paused = true;
                data.pause_timer.reset();
            }

            if data.map_change_requested && data.screen_dimmer.just_dimmed {
                data.map_change_requested = false;
                data.current_room.despawn(&mut ecs);
                data.current_room = Room::new(data.maps.len(), rand::gen_range(19., 20.));
                let new_player_pos = data.spawn_map_entities(&mut ecs);
                data.current_room.started = true;
                let players = ecs.check_components(|e, comps| {
                    comps.player_data.contains_key(e) && comps.positions.contains_key(e)
                });
                for player_e in &players {
                    let pos = ecs.components.positions.get_mut(player_e).unwrap();
                    *pos = new_player_pos;
                }
            }
            if data.pause_timer.just_completed() && !data.show_pause_menu {
                data.paused = false;
            }

            if data.current_room.completed && !data.current_room.upgrade_chosen {
                upgrade_screen.visible = true;
                data.paused = true;
            }

            data.current_map().draw_base();

            if !data.paused {
                spawn_creatures(&mut data, &mut ecs);
                update_timers(&mut ecs);
                update_damageables(&mut ecs);
                damage_on_collision(&ecs, &mut damage_events, &collisions);
                despawn_on_collision(&mut data, &mut ecs, &collisions);
                apply_damage(&mut data, &mut ecs, &mut damage_events);
                kill_entities(&mut ecs, &mut death_events);
                handle_death(&mut data, &mut ecs, &death_events);
                update_player(&mut data, &mut ecs);
                update_weapon(&mut ecs, &mut data);
                update_enemies(&mut data, &mut ecs);
                update_animated_sprites(&mut ecs);
                collisions = move_entities(&mut data, &mut ecs);

                flash_on_damage(&mut ecs);
            }
            draw_animated_sprites(&mut ecs, &data);
            data.current_map().draw_upper();

            data.screen_dimmer.update();
            let dim_progress = if data.screen_dimmer.dimming {
                1. - data.screen_dimmer.progress()
            } else {
                data.screen_dimmer.progress()
            };
            draw_rectangle_ex(
                0.,
                0.,
                360.,
                240.,
                DrawRectangleParams {
                    color: Color::from_rgba(0, 0, 0, (dim_progress * 255.) as u8),
                    ..Default::default()
                },
            );

            if data.debug_collisions {
                draw_colliders(&data, &ecs);
                data.current_map().draw_colliders();
            }

            hud_hearts.draw(&data, &ecs);
            aberration_meter.draw(&data, &ecs);
        }
        if data.state == GameState::Intro {
            if intro_screen.update_and_draw(&mut data) {
                reset_game(&mut data, &mut ecs);
                data.state = GameState::Playing;
                // TODO: reset
            }
        }
        if data.dead {
            data.death_screen.update();
            if data.death_screen.draw(&data) {
                data.state = GameState::Intro;
                data.dead = false;
            }
        }

        if data.show_fps {
            fps_counter.update_and_draw(&mut data);
        }

        if data.show_pause_menu {
            if data.paused && pause_menu(&mut data) {
                break;
            }
        } else if upgrade_screen.visible {
            if let Some(upgrade) = upgrade_screen.draw(&mut data) {
                match upgrade {
                    Upgrade::Item(ref _item) => {}
                    Upgrade::CommonUpgrade(ref _upgrade) => {}
                    Upgrade::Weapon(ref weapon) => match weapon {
                        WeaponType::Launcher => {
                            data.weapon = Weapon::Launcher(Launcher::new());
                        }
                        WeaponType::Balls => {
                            data.weapon = Weapon::Balls(Balls::new());
                        }
                        WeaponType::Dash => {
                            data.weapon = Weapon::Dash(Dash::new());
                        }
                    },
                    Upgrade::WeaponUpgrade(ref upgrade) => match upgrade {
                        WeaponUpgrade::Launcher(ref upgrade) => {
                            if let Weapon::Launcher(ref mut launcher) = data.weapon {
                                launcher.upgrades.push(upgrade.clone());
                            }
                        }
                        WeaponUpgrade::Balls(ref upgrade) => {
                            if let Weapon::Balls(ref mut balls) = data.weapon {
                                balls.upgrades.push(upgrade.clone());
                            }
                        }
                        WeaponUpgrade::Dash(ref upgrade) => {
                            if let Weapon::Dash(ref mut dash) = data.weapon {
                                dash.upgrades.push(upgrade.clone());
                            }
                        }
                    },
                }

                data.map_change_requested = true;
                data.screen_dimmer.dim();
                data.paused = true;
                data.pause_timer.reset();
                data.current_room.upgrade_chosen = true;

                upgrade_screen.visible = false;
                data.paused = false;
            }
        };

        if let Some(render_target) = &mut data.camera.render_target {
            // post_processing_material.set_uniform("intensity", 3f32);
            // post_processing_material.set_uniform("time", get_time() as f32);
            // post_processing_material.set_uniform("hue_shift", 1.8f32);
            set_default_camera();
            gl_use_material(&post_processing_material);
            draw_texture_ex(
                &render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        }
        gl_use_default_material();

        next_frame().await
    }
}

pub async fn pub_main() {
    main();
}
