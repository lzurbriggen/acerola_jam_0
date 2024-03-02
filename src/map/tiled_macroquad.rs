use std::collections::HashMap;
use std::path::PathBuf;
use std::{io::Cursor, path::Path};

use macroquad::prelude::*;
use tiled::Loader;
use tiled::ResourceReader;

struct TiledReader;

impl tiled::ResourceReader for TiledReader {
    type Resource = Cursor<&'static [u8]>;
    type Error = std::io::Error;

    // really dumb example implementation that just keeps resources in memory
    fn read_from(
        &mut self,
        path: &std::path::Path,
    ) -> std::result::Result<Self::Resource, Self::Error> {
        Ok(Cursor::new(include_bytes!("../../assets/map/map_01.tmx")))
    }
}

pub struct TiledMap {
    pub map: tiled::Map,
    tileset_image_cache: HashMap<String, Texture2D>,
    // batch_cache: Option<HashMap<u32, Vec<InstanceArray>>>,
}

impl TiledMap {
    pub async fn new(path: &Path) -> Self {
        let mut loader = Loader::new();

        let map = loader.load_tmx_map(path).unwrap();

        let mut tileset_image_cache = HashMap::new();
        for ts in map.tilesets().iter() {
            if let Some(image) = &ts.image {
                // let img = graphics::Image::from_path(ctx, &image.source)?;
                let img = load_texture(image.source.to_str().unwrap()).await.unwrap();

                tileset_image_cache.insert(ts.name.clone(), img);
            }
        }

        Self {
            map,
            // batch_cache: None,
            tileset_image_cache,
        }
    }

    pub fn load_map(path: &Path) {
        // let mut reader = TiledReader;
        // let map = reader.read_from(path);

        let mut loader = Loader::new();

        let map = loader.load_tmx_map(path).unwrap();

        for layer in map.layers() {
            print!("Layer \"{}\":\n\t", layer.name);

            match layer.layer_type() {
                tiled::LayerType::Tiles(layer) => match layer {
                    tiled::TileLayer::Finite(data) => {
                        for x in 0..data.width() as i32 {
                            for y in 0..data.height() as i32 {
                                if let Some(tile) = data.get_tile(x, y) {
                                    // Get tile's rectangle in the tileset texture
                                    let ts = tile.get_tileset();
                                    println!("{:?} {:?}", tile.id(), ts.name);

                                    // if let Some((batch, ts_size)) =
                                    //     ts_sizes_and_batches.get_mut(&ts.name)
                                    // {
                                    //     let mut dx = x as f32 * self.map.tile_width as f32
                                    //         + parallax_pan.0 * (layer.parallax_x - 1.0);
                                    //     let mut dy = y as f32 * self.map.tile_height as f32
                                    //         + parallax_pan.1 * (layer.parallax_y - 1.0);

                                    //     if self.example_animate {
                                    //         dx += (secs_since_start - x as f32 * 0.3
                                    //             + i as f32 * 0.25)
                                    //             .sin()
                                    //             * 20.0;
                                    //         dy += (secs_since_start * 1.25
                                    //             + y as f32 * 0.3
                                    //             + i as f32 * 0.25)
                                    //             .cos()
                                    //             * 20.0;
                                    //     }

                                    //     batch.push(
                                    //         DrawParam::default()
                                    //             .src(get_tile_rect(
                                    //                 ts,
                                    //                 tile.id(),
                                    //                 ts_size.0,
                                    //                 ts_size.1,
                                    //             ))
                                    //             .dest([dx, dy])
                                    //             .color(ggez::graphics::Color::from_rgba(
                                    //                 0xFF,
                                    //                 0xFF,
                                    //                 0xFF,
                                    //                 (layer.opacity * 255.0) as u8,
                                    //             )),
                                    //     );
                                    // }
                                }
                            }
                        }
                    }
                    tiled::TileLayer::Infinite(data) => {
                        println!(
                            "Infinite tile layer; Tile @ (-5, 0) = {:?}",
                            data.get_tile(-5, 0)
                        )
                    }
                },
                tiled::LayerType::Objects(layer) => {
                    println!("Object layer with {} objects", layer.objects().len())
                }
                tiled::LayerType::Image(layer) => {
                    println!(
                        "Image layer with {}",
                        match &layer.image {
                            Some(img) =>
                                format!("an image with source = {}", img.source.to_string_lossy()),
                            None => "no image".to_owned(),
                        }
                    )
                }
                tiled::LayerType::Group(layer) => {
                    println!("Group layer with {} sublayers", layer.layers().len())
                }
            }
        }
    }
}
