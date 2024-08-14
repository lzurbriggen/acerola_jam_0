use std::collections::HashMap;

use entity::{
    entities::Ecs,
    entity_id::Entity,
    events::{DamageEvent, DeathEvent},
    mirituhg::spawn_mirituhg,
    player::spawn_player,
    upgrades::{CommonUpgrade, ItemUpgrade, Upgrade, WeaponUpgrade},
};
use fps_counter::FPSCounter;
use game_data::{Audio, GameMaterial};
use game_state::GameState;
use items::weapon::{Balls, Dash, Launcher, Weapon, WeaponType};
use macroquad::{
    audio::{self, play_sound, set_sound_volume},
    miniquad::window::set_mouse_cursor,
    prelude::*,
};
use macroquad_tiled::load_map;
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
    mirituhg::HudMirituhg,
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
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            framebuffer_alpha: false,
            swap_interval: None,
            ..Default::default()
        },
        icon: Some(icon::set()),
        sample_count: 0,
        ..Default::default()
    }
}

fn load_texture_bytes(bytes: &[u8]) -> Texture2D {
    let image = Image::from_file_with_format(bytes, None).unwrap();
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let mut font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Bitfantasy.ttf")).unwrap();
    font.set_filter(FilterMode::Nearest);

    let mut icon_font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Zicons.ttf")).unwrap();
    icon_font.set_filter(FilterMode::Nearest);

    // UI assets
    let button_texture = load_texture_bytes(include_bytes!("../assets/ui/button_bg.png"));
    let button_texture_hover =
        load_texture_bytes(include_bytes!("../assets/ui/button_bg_hover.png"));
    button_texture_hover.set_filter(FilterMode::Nearest);
    let button_texture_pressed =
        load_texture_bytes(include_bytes!("../assets/ui/button_bg_clicked.png"));
    button_texture_pressed.set_filter(FilterMode::Nearest);
    let frame_texture = load_texture_bytes(include_bytes!("../assets/ui/window_bg.png"));
    frame_texture.set_filter(FilterMode::Nearest);
    let frame_texture_pretty =
        load_texture_bytes(include_bytes!("../assets/ui/window_bg_pretty.png"));
    frame_texture_pretty.set_filter(FilterMode::Nearest);
    let focus_bg_texture = load_texture_bytes(include_bytes!("../assets/ui/focus_bg.png"));
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

    let hud_heart_texture = load_texture_bytes(include_bytes!("../assets/ui/heart_01.png"));
    let hopper_texture = load_texture_bytes(include_bytes!("../assets/entities/hopper_01.png"));
    let spitter_texture = load_texture_bytes(include_bytes!("../assets/entities/spitter.png"));
    let stomper_texture = load_texture_bytes(include_bytes!("../assets/entities/stomper.png"));
    let mirituhg_texture = load_texture_bytes(include_bytes!("../assets/entities/mirituhg.png"));
    let skull_texture = load_texture_bytes(include_bytes!("../assets/entities/skull_01.png"));
    let bullet_texture = load_texture_bytes(include_bytes!("../assets/entities/bullet_01.png"));
    let bullet_enemy_texture =
        load_texture_bytes(include_bytes!("../assets/entities/bullet_enemy.png"));
    let dust_texture = load_texture_bytes(include_bytes!("../assets/entities/dust_01.png"));
    let blood_texture = load_texture_bytes(include_bytes!("../assets/entities/blood_01.png"));
    let aberration_meter_texture =
        load_texture_bytes(include_bytes!("../assets/ui/aberration_meter.png"));
    let noise1_texture =
        load_texture_bytes(include_bytes!("../assets/entities/Perlin_16-128x128.png"));
    let noise2_texture =
        load_texture_bytes(include_bytes!("../assets/entities/Perlin_15-128x128.png"));
    let aberration_meter_mask_texture =
        load_texture_bytes(include_bytes!("../assets/ui/aberration_meter_mask.png"));
    let death_texture = load_texture_bytes(include_bytes!("../assets/ui/death.png"));
    let end_game_screen_texture =
        load_texture_bytes(include_bytes!("../assets/ui/end_game_screen.png"));
    let player_texture = load_texture_bytes(include_bytes!("../assets/entities/player_01.png"));
    let intro_screen_texture = load_texture_bytes(include_bytes!("../assets/ui/intro_screen.png"));
    let health_texture = load_texture_bytes(include_bytes!("../assets/entities/health.png"));
    let anomaly_big_texture =
        load_texture_bytes(include_bytes!("../assets/entities/anomaly_big_01.png"));
    let anomaly_small_texture =
        load_texture_bytes(include_bytes!("../assets/entities/anomaly_small_01.png"));
    let upgrade_frame_inner_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_frame_inner.png"));
    let upgrade_frame_inner_dark_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_frame_inner_dark.png"));
    let upgrade_banner_item_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_banner_item.png"));
    let upgrade_banner_upgrade_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_banner_upgrade.png"));
    let upgrade_banner_weapon_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_banner_weapon.png"));
    let upgrade_launcher_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_launcher.png"));
    let upgrade_balls_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_balls.png"));
    let upgrade_dash_texture = load_texture_bytes(include_bytes!("../assets/ui/upgrade_dash.png"));
    let upgrade_common_max_hp_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_common_max_hp.png"));
    let upgrade_item_hp_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_item_hp.png"));
    let upgrade_items_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_items.png"));
    let upgrade_move_speed_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_move_speed.png"));
    let upgrade_item_anomaly_big_texture =
        load_texture_bytes(include_bytes!("../assets/ui/upgrade_item_anomaly_big.png"));
    let upgrade_item_anomaly_small_texture = load_texture_bytes(include_bytes!(
        "../assets/ui/upgrade_item_anomaly_small.png"
    ));

    let overlay_mirituhg_texture =
        load_texture_bytes(include_bytes!("../assets/ui/overlay_mirituhg.png"));

    let boss_health_bar_texture = load_texture_bytes(include_bytes!("../assets/ui/health_bar.png"));

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
        ("mirituhg", mirituhg_texture),
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
        ("upgrade_move_speed", upgrade_move_speed_texture),
        ("upgrade_items", upgrade_items_texture),
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
    let ui_switch_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/touch_4.wav"
    ))
    .await
    .unwrap();
    let shoot_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/hurt_1.wav"
    ))
    .await
    .unwrap();
    let death_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/expl_2.wav"
    ))
    .await
    .unwrap();
    let death2_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/down.wav"
    ))
    .await
    .unwrap();
    let spawn_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/touch_4.wav"
    ))
    .await
    .unwrap();
    let kill_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/touch_4.wav"
    ))
    .await
    .unwrap();
    let confirm_sfx =
        audio::load_sound_from_bytes(include_bytes!("../assets/audio/ui/bookClose.ogg"))
            .await
            .unwrap();
    let confirm2_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/jump_5.wav"
    ))
    .await
    .unwrap();
    let hit_sfx = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/mini_sounds_pack_40/hurt_5.wav"
    ))
    .await
    .unwrap();
    let hit2_sfx =
        audio::load_sound_from_bytes(include_bytes!("../assets/audio/mini_sounds_pack_40/2.wav"))
            .await
            .unwrap();

    // Music
    let music1 = audio::load_sound_from_bytes(include_bytes!(
        "../assets/audio/music/game_240308_mirituhg_battle 2024-03-14 1740.ogg"
    ))
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
    let tileset = load_texture_bytes(include_bytes!("../assets/map/tileset_01.png"));

    let tiled_map1_json = include_str!("../assets/map/example_01.tmj");
    let tiled_map1 =
        load_map(tiled_map1_json, &[("tileset_01.png", tileset.clone())], &[]).unwrap();
    entity_index += 1;
    let map1 = Map::new(Entity(entity_index), &settings, tiled_map1);

    let tiled_map2_json = include_str!("../assets/map/map2.tmj");
    let tiled_map2 =
        load_map(tiled_map2_json, &[("tileset_01.png", tileset.clone())], &[]).unwrap();
    entity_index += 1;
    let map2 = Map::new(Entity(entity_index), &settings, tiled_map2);

    let tiled_map3_json = include_str!("../assets/map/map3.tmj");
    let tiled_map3 =
        load_map(tiled_map3_json, &[("tileset_01.png", tileset.clone())], &[]).unwrap();
    entity_index += 1;
    let map3 = Map::new(Entity(entity_index), &settings, tiled_map3);

    let tiled_map4_json = include_str!("../assets/map/map4.tmj");
    let tiled_map4 =
        load_map(tiled_map4_json, &[("tileset_01.png", tileset.clone())], &[]).unwrap();
    entity_index += 1;
    let map4 = Map::new(Entity(entity_index), &settings, tiled_map4);

    let maps = vec![map4, map1, map2, map3];

    let mut data = GameData::new(
        entity_index,
        settings,
        ui_data,
        maps,
        graphics,
        audio,
        death_texture,
        end_game_screen_texture,
    );
    data.reset();
    data.settings.set_window_size(WindowSize::W1440);

    let mut ecs = Ecs::default();

    let mut collisions = HashMap::new();
    let mut damage_events = Vec::<DamageEvent>::new();
    let mut death_events = Vec::<DeathEvent>::new();

    let hud_hearts = HudHearts::new(&data);
    let hud_mirituhg = HudMirituhg::new(overlay_mirituhg_texture, boss_health_bar_texture);
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

    let mut upgrade_screen = UpgradeScreen::new(vec![]);

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

        if data.state == GameState::Playing {
            if is_key_pressed(KeyCode::F5) {
                // Reset?
                // reset_game(&mut data, &mut ecs);
                spawn_mirituhg(&mut data, vec2(180., 120.), &mut ecs);
            }

            if is_key_pressed(KeyCode::F3) {
                upgrade_screen.visible = true;
                data.paused = true;
            }

            let despawned_entities = &ecs.marked_for_despawn.clone();
            for entity in despawned_entities {
                let entity_i = ecs.entities.iter().position(|e| e == entity);
                if let Some(index) = entity_i {
                    ecs.entities.remove(index);
                    ecs.remove_all_components(entity);
                }
            }
            // TODO: Hack
            if despawned_entities.len() > 0 {
                data.current_room.check_completed(&ecs);
            }

            if !data.paused {
                ecs.marked_for_despawn.clear();
                death_events.clear();
            }
        }
        data.graphics
            .aberration_meter_material
            .set_uniform("time", get_time() as f32);

        post_processing_material.set_texture("noise1", data.graphics.noise1_texture.clone());
        post_processing_material.set_texture("noise2", data.graphics.noise2_texture.clone());

        set_mouse_cursor(miniquad::CursorIcon::Default);

        data.input.update(&ecs, &data.camera);
        clear_background(Color::from_hex(0x060608));

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
            if is_key_pressed(KeyCode::F6) {
                data.next_room(&mut ecs);
            }

            // Map transition
            if data.map_change_requested && data.screen_dimmer.just_dimmed {
                if let Some(next_room) = data.next_room {
                    data.map_change_requested = false;
                    data.current_room.despawn(&mut ecs);
                    data.next_room = None;
                    data.current_room = next_room;
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
            }
            if data.pause_timer.just_completed() && !data.show_pause_menu && !upgrade_screen.visible
            {
                data.paused = false;
            }

            data.current_map().draw_base();

            if !data.paused {
                spawn_creatures(&mut data, &mut ecs);
                update_timers(&mut ecs);
                update_damageables(&mut ecs);
                damage_on_collision(&ecs, &mut damage_events, &collisions);
                despawn_on_collision(&mut data, &mut ecs, &collisions);
                kill_entities(&data, &mut ecs, &mut death_events);
                handle_death(&mut data, &mut ecs, &death_events);
                update_player(&mut data, &mut ecs);
                update_weapon(&mut ecs, &mut data);
                update_enemies(&mut data, &mut ecs, &mut damage_events);
                apply_damage(&mut data, &mut ecs, &mut damage_events);
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
            hud_mirituhg.draw(&data, &ecs);
        }
        if data.state == GameState::Intro {
            if intro_screen.update_and_draw(&mut data) {
                data.reset();
                spawn_player(&mut data, &mut ecs);
                data.next_room(&mut ecs);
                data.state = GameState::Playing;
                // TODO: reset
            }
        }
        if data.dead {
            data.death_screen.update();
            if data.death_screen.draw(&data) {
                data.reset();
            }
        }
        if data.game_completed {
            data.end_game_screen.update();
            if data.end_game_screen.draw(&data) {
                data.state = GameState::Intro;
                data.game_completed = false;
            }
        }

        if data.current_room.started {
            if !data.current_room.upgrade_chosen {
                upgrade_screen.visible = true;
            }
        }

        if data.show_fps {
            fps_counter.update_and_draw(&mut data);
        }

        if data.current_room.completed && !data.map_change_requested && !data.game_completed {
            data.next_room(&mut ecs);
        }

        if data.paused {
            if data.show_pause_menu {
                if pause_menu(&mut data) {
                    break;
                }
            } else if upgrade_screen.visible {
                upgrade_screen.upgrades = data.current_room.available_upgrades.clone();
                if let Some(upgrade) = upgrade_screen.draw(&mut data) {
                    let players =
                        ecs.check_components(|e, comps| comps.player_data.contains_key(e));
                    let player_data = ecs.components.player_data.get_mut(&players[0]).unwrap();
                    let health = ecs.components.health.get_mut(&players[0]).unwrap();

                    match upgrade {
                        Upgrade::Item(ref item) => match item {
                            ItemUpgrade::Hp(hp) => {
                                health.hp += *hp;
                            }
                            ItemUpgrade::AnomalySmall => {
                                player_data.aberration = (player_data.aberration - 0.1).max(0.);
                            }
                            ItemUpgrade::AnomalyBig => {
                                player_data.aberration = (player_data.aberration - 0.5).max(0.);
                            }
                        },
                        Upgrade::CommonUpgrade(ref upgrade) => match upgrade {
                            CommonUpgrade::MaxHp(hp) => {
                                health.hp += *hp as f32;
                                player_data.upgrades.push(upgrade.clone())
                            }
                            CommonUpgrade::MoveSpeed(_) => {
                                player_data.upgrades.push(upgrade.clone())
                            }
                            CommonUpgrade::ItemDropChance(increase) => {
                                data.item_drop_chance_increase += increase;
                            }
                        },
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

                    data.current_room.upgrade_chosen = true;

                    upgrade_screen.visible = false;
                    data.paused = false;
                }
            };
        }

        if let Some(render_target) = &mut data.camera.render_target {
            post_processing_material.set_uniform("intensity", 0.21f32);
            post_processing_material.set_uniform("time", get_time() as f32);
            post_processing_material.set_uniform("texture_size", render_target.texture.size());
            let players = ecs.check_components(|e, comps| comps.player_data.contains_key(e));
            if players.len() > 0 {
                if let Some(player_data) = ecs.components.player_data.get_mut(&players[0]) {
                    post_processing_material
                        .set_uniform("hue_shift", player_data.aberration * 0.01);
                    post_processing_material.set_uniform("intensity", player_data.aberration + 0.2);
                }
            }

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
