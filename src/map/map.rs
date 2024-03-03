use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;
use tiled::TileId;

pub struct Map {
    pub tiled_map: TiledMap,
    pub tileset_collision_map: HashMap<String, HashSet<TileId>>,
    pub map_collision: HashSet<(usize, usize)>,
    map_rect: Rect,
}

impl Map {
    pub fn new(map: TiledMap, resolution: Vec2) -> Self {
        let mut tileset_collision_map = HashMap::<String, HashSet<TileId>>::new();
        for tileset in &map.raw_tiled_map.tilesets {
            let mut collision = HashSet::<TileId>::new();
            for tile in &tileset.tiles {
                let has_collision = tile.properties.iter().any(|prop| prop.name == "collision");
                if has_collision {
                    collision.insert(tile.id as TileId);
                }
            }
            tileset_collision_map.insert(tileset.name.clone(), collision);
        }

        let mut map_collision = HashSet::<(usize, usize)>::new();
        for layer in &map.layers {
            let layer_width = layer.1.width;
            for (tile_index, tile) in layer.1.data.iter().enumerate() {
                if let Some(tile) = tile {
                    let ts = tileset_collision_map.get(&tile.tileset);
                    if let Some(ts) = ts {
                        if ts.contains(&tile.id) {
                            let tile_x = tile_index % layer_width as usize;
                            let tile_y = tile_index / layer_width as usize;
                            map_collision.insert((tile_x, tile_y));
                        }
                    }
                }
            }
        }

        Self {
            tiled_map: map,
            tileset_collision_map,
            map_collision,
            map_rect: Rect::new(0., 0., resolution.x, resolution.y),
        }
    }

    pub fn draw_base(&self) {
        self.tiled_map.draw_tiles("layer0", self.map_rect, None);
        self.tiled_map.draw_tiles("layer1", self.map_rect, None);
        self.tiled_map.draw_tiles("layer2", self.map_rect, None);
    }

    pub fn draw_upper(&self) {
        self.tiled_map.draw_tiles("layer3", self.map_rect, None);
    }

    pub fn draw_colliders(&self) {
        for (x, y) in &self.map_collision {
            draw_rectangle_lines(*x as f32 * 8., *y as f32 * 8., 8., 8., 1., GREEN);
        }
    }
}
