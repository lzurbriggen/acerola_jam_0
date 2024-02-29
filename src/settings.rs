use macroquad::{miniquad::window, prelude::*};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum WindowSize {
    W360,
    #[default]
    W740,
    W1440,
    Fullscreen,
}

impl WindowSize {
    pub fn text(&self) -> String {
        match self.size() {
            None => "Fullscreen".to_string(),
            Some(size) => {
                format!("{}*{}", size.0, size.1)
            }
        }
    }

    pub fn size(&self) -> Option<(u32, u32)> {
        match self {
            WindowSize::W360 => Some((360, 240)),
            WindowSize::W740 => Some((740, 480)),
            WindowSize::W1440 => Some((1440, 960)),
            WindowSize::Fullscreen => None,
        }
    }

    pub fn list() -> Vec<WindowSize> {
        vec![
            WindowSize::W360,
            WindowSize::W740,
            WindowSize::W1440,
            WindowSize::Fullscreen,
        ]
    }
}

pub struct GameSettings {
    pub sfx_volume_lin: f32,
    pub sfx_volume: f32,
    pub music_volume_lin: f32,
    pub music_volume: f32,
    pub window_size: WindowSize,
}

impl Default for GameSettings {
    fn default() -> Self {
        let mut settings = Self {
            sfx_volume_lin: Default::default(),
            sfx_volume: Default::default(),
            music_volume_lin: Default::default(),
            music_volume: Default::default(),
            window_size: Default::default(),
        };
        settings.set_music_volume_lin(0.75);
        settings.set_sfx_volume_lin(0.75);

        settings
    }
}

impl GameSettings {
    pub fn set_sfx_volume_lin(&mut self, vol: f32) {
        self.sfx_volume_lin = vol;
        self.sfx_volume = (f32::exp(6.908 * self.sfx_volume_lin) / 1000.).clamp(0., 1.);
    }

    pub fn set_music_volume_lin(&mut self, vol: f32) {
        self.music_volume_lin = vol;
        self.music_volume = (f32::exp(6.908 * self.music_volume_lin) / 1000.).clamp(0., 1.);
    }

    pub fn set_window_size(&mut self, size: WindowSize) {
        self.window_size = size;
        if let Some(size) = size.size() {
            window::set_fullscreen(false);
            window::set_window_size(size.0, size.1);
            return;
        }
        window::set_fullscreen(true);
    }
}
