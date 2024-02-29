use fps_counter::FPSCounter;
use game_state::GameState;
use macroquad::{audio, miniquad::window::set_mouse_cursor, prelude::*};
use settings::{GameSettings, WindowSize};
use ui::{pause_menu::pause_menu, ui_data::UIData};

mod fps_counter;
mod game_state;
mod settings;
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

    let mut font = load_ttf_font("./fonts/Bitfantasy.ttf").await.unwrap();
    font.set_filter(FilterMode::Nearest);

    let mut icon_font = load_ttf_font("./fonts/Zicons.ttf").await.unwrap();
    icon_font.set_filter(FilterMode::Nearest);

    let button_texture: Texture2D = load_texture("./ui/button_bg.png").await.unwrap();
    button_texture.set_filter(FilterMode::Nearest);
    let button_texture_hover: Texture2D = load_texture("./ui/button_bg_hover.png").await.unwrap();
    button_texture_hover.set_filter(FilterMode::Nearest);
    let button_texture_pressed: Texture2D =
        load_texture("./ui/button_bg_clicked.png").await.unwrap();
    button_texture_pressed.set_filter(FilterMode::Nearest);
    let frame_texture: Texture2D = load_texture("./ui/window_bg.png").await.unwrap();
    frame_texture.set_filter(FilterMode::Nearest);

    let button_click_sfx = audio::load_sound("audio/ui/bookClose.ogg").await.unwrap();

    let ui_data = UIData {
        button_texture: button_texture,
        button_texture_hover: button_texture_hover,
        button_texture_pressed: button_texture_pressed,
        button_click_sfx: button_click_sfx,
        frame_texture: frame_texture,
        font: font.clone(),
        icon_font: icon_font.clone(),
        text_color: Color::from_hex(0xe4d2aa),
        text_shadow_color: Color::from_hex(0xb4202a),
    };
    let mut settings = GameSettings::default();

    let mut fullscreen = false;
    let mut nearest = false;

    let mut camera = Camera2D::from_display_rect(Rect {
        x: 0.,
        y: 240.,
        w: 360.,
        h: -240.,
    });
    camera.render_target = Some(render_target.clone());

    let mut fps_counter = FPSCounter::default();

    let mut game_state = GameState::default();
    let mut paused = false;

    loop {
        set_mouse_cursor(miniquad::CursorIcon::Default);
        set_camera(&camera);

        clear_background(RED);

        if (is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt))
            && is_key_pressed(KeyCode::Enter)
        {
            fullscreen = !fullscreen;

            if fullscreen {
                settings.set_window_size(settings::WindowSize::Fullscreen);
            } else {
                settings.set_window_size(WindowSize::default());
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            paused = !paused;
        }

        fps_counter.update_and_draw(&ui_data);

        if paused && pause_menu(&ui_data, &mut settings) {
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
