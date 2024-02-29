use assets::Assets;
use macroquad::{
    audio,
    miniquad::window::set_mouse_cursor,
    prelude::*,
    time,
    ui::{root_ui, Skin},
};
use pause_menu::pause_menu;
use settings::GameSettings;
use ui::ui_data::{UIDataGlobal, UI_DATA};

mod assets;
mod game_state;
mod pause_menu;
mod settings;
mod ui;

fn window_conf() -> Conf {
    Conf {
        window_title: "Test".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1440,
        window_height: 960,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            framebuffer_alpha: false,
            ..Default::default()
        },
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(360, 240);
    render_target.texture.set_filter(FilterMode::Nearest);

    let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/fonts/Bitfantasy.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(32)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../assets/ui/window_bg.png"), None)
                    .unwrap(),
            )
            .background_margin(RectOffset::new(3.0, 3.0, 3.0, 3.0))
            .margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .font(include_bytes!("../assets/fonts/Bitfantasy.ttf"))
            .unwrap()
            .background(
                Image::from_file_with_format(include_bytes!("../assets/ui/button_bg.png"), None)
                    .unwrap(),
            )
            .background_margin(RectOffset::new(2.0, 2.0, 2.0, 2.0))
            .margin(RectOffset::new(16.0, 0.0, 6.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui/button_bg_hover.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui/button_bg_clicked.png"),
                    None,
                )
                .unwrap(),
            )
            .text_color(Color::from_hex(0x322b28))
            .text_color_hovered(Color::from_hex(0x5a4e44))
            .text_color_clicked(Color::from_hex(0xe3e6ff))
            .build();

        Skin {
            // editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };
    root_ui().push_skin(&skin1);

    let mut settings = GameSettings::default();

    let mut font = load_ttf_font("./assets/fonts/Bitfantasy.ttf")
        .await
        .unwrap();
    font.set_filter(FilterMode::Nearest);

    let mut icon_font = load_ttf_font("./assets/fonts/Zicons.ttf").await.unwrap();
    icon_font.set_filter(FilterMode::Nearest);

    set_pc_assets_folder("assets");

    let sound1 = audio::load_sound("audio/ui/bookClose.ogg").await.unwrap();

    let mut assets = Assets::default();
    assets.sfx.insert("test_sound".to_string(), sound1.clone());

    settings.set_sfx_volume_lin(0.75, &mut assets);
    settings.set_music_volume_lin(0.75, &mut assets);

    let texture: Texture2D = load_texture("mockup_01.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let button_texture: Texture2D = load_texture("./ui/button_bg.png").await.unwrap();
    button_texture.set_filter(FilterMode::Nearest);
    let button_texture_hover: Texture2D = load_texture("./ui/button_bg_hover.png").await.unwrap();
    button_texture_hover.set_filter(FilterMode::Nearest);
    let button_texture_pressed: Texture2D =
        load_texture("./ui/button_bg_clicked.png").await.unwrap();
    button_texture_pressed.set_filter(FilterMode::Nearest);
    let frame_texture: Texture2D = load_texture("./ui/window_bg.png").await.unwrap();
    frame_texture.set_filter(FilterMode::Nearest);

    unsafe {
        UI_DATA = UIDataGlobal {
            button_texture: Some(button_texture),
            button_texture_hover: Some(button_texture_hover),
            button_texture_pressed: Some(button_texture_pressed),
            button_click_sfx: Some(sound1),
            frame_texture: Some(frame_texture),
            font: Some(font.clone()),
            icon_font: Some(icon_font.clone()),
            text_color: Color::from_hex(0xe4d2aa),
            text_shadow_color: Color::from_hex(0xb4202a),
        };
    }

    let mut fullscreen = false;
    let mut nearest = false;

    let mut camera = Camera2D::from_display_rect(Rect {
        x: 0.,
        y: 240.,
        w: 360.,
        h: -240.,
    });
    camera.render_target = Some(render_target.clone());

    loop {
        set_mouse_cursor(miniquad::CursorIcon::Default);
        set_camera(&camera);

        let delta = time::get_frame_time();

        clear_background(RED);

        if (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
            && is_key_pressed(KeyCode::Enter)
        {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }

        if is_key_pressed(KeyCode::E) {
            nearest = !nearest;
            if nearest {
                texture.set_filter(FilterMode::Linear);
            } else {
                texture.set_filter(FilterMode::Linear);
            }
        }

        // draw_texture(&texture, 0., 0., WHITE);
        // draw_texture_ex(
        //     &texture,
        //     0.,
        //     0.,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(256., 256.)),
        //         ..Default::default()
        //     },
        // );

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text_ex(
            format!("FPS: {}", time::get_fps()).as_str(),
            8.0,
            16.0,
            TextParams {
                font: Some(&font),
                font_size: 16,
                ..Default::default()
            },
        );

        if pause_menu(&mut assets, &mut settings) {
            break;
        }

        set_default_camera();
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

        next_frame().await
    }
}
