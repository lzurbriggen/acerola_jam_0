use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;

pub mod tiled_macroquad;

pub struct Map {
    pub tiled_map: TiledMap,
}

impl Map {
    pub fn draw(&self) {
        for (layer_name, _) in &self.tiled_map.layers {
            self.tiled_map
                .draw_tiles(layer_name, Rect::new(0., 0., 360., 240.), None);
        }
    }
}
