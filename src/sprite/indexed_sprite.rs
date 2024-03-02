use macroquad::prelude::*;

pub struct IndexedSprite {
    texture: Texture2D,
    frame_width: u8,
    row_len: usize,
}

impl IndexedSprite {
    pub fn new(texture: Texture2D, frame_width: u8) -> Self {
        let row_len = (texture.width() / frame_width as f32).trunc() as usize;
        Self {
            texture,
            frame_width,
            row_len,
        }
    }

    pub fn draw(&self, pos: Vec2, index: usize) {
        draw_texture_ex(
            &self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.texture_source(index)),
                ..Default::default()
            },
        )
    }

    pub fn texture_source(&self, index: usize) -> Rect {
        let x = index % self.row_len * self.frame_width as usize;
        let y = index / self.row_len * self.frame_width as usize;
        Rect::new(
            x as f32,
            y as f32,
            self.frame_width as f32,
            self.frame_width as f32,
        )
    }
}
