use macroquad::{audio::Sound, prelude::*};

pub struct UIData {
    pub button_texture: Texture2D,
    pub button_texture_hover: Texture2D,
    pub button_texture_pressed: Texture2D,
    pub button_click_sfx: Sound,
    pub frame_texture: Texture2D,
    pub frame_texture_pretty: Texture2D,
    pub focus_background_texture: Texture2D,
    pub font: Font,
    pub icon_font: Font,
    pub text_color: Color,
    pub text_shadow_color: Color,
    pub focus: Option<u64>,
}
