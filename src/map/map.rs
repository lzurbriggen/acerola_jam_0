use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;
use macroquad_tiled::Map as TiledMap;

use crate::{
    entity::{door::spawn_door, entities::Ecs, entity_id::Entity, spawner::spawn_spawner},
    game_data::GameData,
};

pub struct Map {
    pub id: Entity,
    pub tiled_map: TiledMap,
    pub tileset_collision_map: HashMap<String, HashSet<usize>>,
    pub map_collision: HashSet<(usize, usize)>,
    map_rect: Rect,
}

impl Map {
    pub fn new(data: &mut GameData, map: TiledMap) -> Self {
        let id = data.new_entity();

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
            map_rect: Rect::new(
                0.,
                0.,
                data.settings.resolution.x,
                data.settings.resolution.y,
            ),
        }
    }

    pub fn spawn_entities(&self, data: &mut GameData, ecs: &mut Ecs) {
        for (_, layer) in &self.tiled_map.layers {
            for object in &layer.objects {
                let object_pos = vec2(object.world_x + 4., object.world_y - 4.);
                if let Some(_door_dir) = object.properties.get("door") {
                    spawn_door(data, object_pos, ecs);
                }
                if let Some(_) = object.properties.get("spawn") {
                    spawn_spawner(data, object_pos, ecs);
                }
            }
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
