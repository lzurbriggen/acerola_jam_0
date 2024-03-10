use macroquad::prelude::*;

use crate::game_data::GameData;

#[derive(Clone)]
pub struct IndexedSprite {
    pub texture: &'static str,
    position_offset: Vec2,
    frame_width: u32,
    row_len: usize,
}

impl IndexedSprite {
    pub fn new(
        data: &GameData,
        texture: &'static str,
        frame_width: u32,
        position_offset: Vec2,
    ) -> Self {
        let tex = data.graphics.textures.get(texture).unwrap();
        let row_len = (tex.width() / frame_width as f32).trunc() as usize;
        Self {
            texture,
            frame_width,
            row_len,
            position_offset,
        }
    }

    pub fn draw(&self, data: &GameData, pos: Vec2, index: usize) {
        self.draw_with_dest(data, pos, index, None);
    }

    pub fn draw_with_dest(
        &self,
        data: &GameData,
        pos: Vec2,
        index: usize,
        destination_size: Option<Vec2>,
    ) {
        let pos = pos - self.position_offset;
        let tex = data.graphics.textures.get(self.texture).unwrap();
        draw_texture_ex(
            &tex,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: destination_size,
                source: Some(self.texture_source(data, index)),
                ..Default::default()
            },
        )
    }

    pub fn texture_source(&self, data: &GameData, index: usize) -> Rect {
        let x = index % self.row_len * self.frame_width as usize;
        let y = index / self.row_len * self.frame_width as usize;
        let tex = data.graphics.textures.get(self.texture).unwrap();
        Rect::new(x as f32, y as f32, self.frame_width as f32, tex.height())
    }
}
