use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;

use crate::game_data::GameData;

pub struct Map {
    pub tiled_map: TiledMap,
}

impl Map {
    pub fn draw_base(&self, data: &GameData) {
        let size = data.settings.resolution;
        self.tiled_map
            .draw_tiles("layer0", Rect::new(0., 0., size.x, size.y), None);
        self.tiled_map
            .draw_tiles("layer1", Rect::new(0., 0., size.x, size.y), None);
        self.tiled_map
            .draw_tiles("layer2", Rect::new(0., 0., size.x, size.y), None);
    }

    pub fn draw_upper(&self, data: &GameData) {
        let size = data.settings.resolution;
        self.tiled_map
            .draw_tiles("layer3", Rect::new(0., 0., size.x, size.y), None);
    }
}
