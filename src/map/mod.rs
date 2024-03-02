use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;

pub mod tiled_macroquad;

pub struct Map {
    pub tiled_map: TiledMap,
}

impl Map {
    pub fn draw_base(&self) {
        // for (layer_name, _) in &self.tiled_map.layers {
        //     println!("{:?}", layer_name);
        //     self.tiled_map
        //         .draw_tiles(layer_name, Rect::new(0., 0., 360., 240.), None);
        // }

        self.tiled_map
            .draw_tiles("layer0", Rect::new(0., 0., 360., 240.), None);
        self.tiled_map
            .draw_tiles("layer1", Rect::new(0., 0., 360., 240.), None);
        self.tiled_map
            .draw_tiles("layer2", Rect::new(0., 0., 360., 240.), None);
    }

    pub fn draw_upper(&self) {
        self.tiled_map
            .draw_tiles("layer3", Rect::new(0., 0., 360., 240.), None);
    }
}
