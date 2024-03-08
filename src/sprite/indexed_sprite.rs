use macroquad::prelude::*;

#[derive(Clone)]
pub struct IndexedSprite {
    // TODO: use reference to texture
    pub texture: Texture2D,
    position_offset: Vec2,
    frame_width: u8,
    row_len: usize,
}

impl IndexedSprite {
    pub fn new(texture: Texture2D, frame_width: u8, position_offset: Vec2) -> Self {
        let row_len = (texture.width() / frame_width as f32).trunc() as usize;
        Self {
            texture,
            frame_width,
            row_len,
            position_offset,
        }
    }

    pub fn draw(&self, pos: Vec2, index: usize) {
        self.draw_with_dest(pos, index, None);
    }

    pub fn draw_with_dest(&self, pos: Vec2, index: usize, destination_size: Option<Vec2>) {
        let pos = pos - self.position_offset;
        draw_texture_ex(
            &self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: destination_size,
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
            self.texture.height(),
        )
    }
}
