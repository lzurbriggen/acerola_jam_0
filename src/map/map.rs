use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;

use crate::{entity::entity_id::Entity, settings::GameSettings};

pub struct Map {
    pub id: Entity,
    pub tiled_map: TiledMap,
    pub tileset_collision_map: HashMap<String, HashSet<usize>>,
    pub map_collision: HashSet<(usize, usize)>,
    map_rect: Rect,
}

impl Map {
    pub fn new(id: Entity, settings: &GameSettings, map: TiledMap) -> Self {
        let mut tileset_collision_map = HashMap::<String, HashSet<usize>>::new();
        for tileset in &map.raw_tiled_map.tilesets {
            let mut collision = HashSet::<usize>::new();
            for tile in &tileset.tiles {
                let has_collision = tile.properties.iter().any(|prop| prop.name == "collision");
                if has_collision {
                    collision.insert(tile.id);
                }
            }
            tileset_collision_map.insert(tileset.name.clone(), collision);
        }

        let mut map_collision = HashSet::<(usize, usize)>::new();
        for (_, layer) in &map.layers {
            let layer_width = layer.width;
            for (tile_index, tile) in layer.data.iter().enumerate() {
                if let Some(tile) = tile {
                    let ts = tileset_collision_map.get(&tile.tileset);
                    if let Some(ts) = ts {
                        if ts.contains(&(tile.id as usize)) {
                            let tile_x = tile_index % layer_width as usize;
                            let tile_y = tile_index / layer_width as usize;
                            map_collision.insert((tile_x, tile_y));
                        }
                    }
                }
            }
        }

        Self {
            id,
            tiled_map: map,
            tileset_collision_map,
            map_collision,
            map_rect: Rect::new(0., 0., settings.resolution.x, settings.resolution.y),
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
