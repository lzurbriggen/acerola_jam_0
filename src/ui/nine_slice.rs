use macroquad::prelude::*;

// TODO: do this in a shader
pub fn nice_slice(texture: &Texture2D, offsets: &RectOffset, rect: &Rect) {
    // top left
    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(0., 0., offsets.left, offsets.top)),
            ..Default::default()
        },
    );

    // top
    draw_texture_ex(
        texture,
        rect.x + offsets.left,
        rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(rect.w - offsets.left - offsets.right, offsets.top)),
            source: Some(Rect::new(
                offsets.left,
                0.,
                texture.width() - offsets.left - offsets.right,
                offsets.top,
            )),
            ..Default::default()
        },
    );

    // top right
    draw_texture_ex(
        texture,
        rect.x + rect.w - offsets.left,
        rect.y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                texture.width() - offsets.left,
                0.,
                offsets.right,
                offsets.top,
            )),
            ..Default::default()
        },
    );

    // left
    draw_texture_ex(
        texture,
        rect.x,
        rect.y + offsets.top,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(offsets.left, rect.h - offsets.top - offsets.bottom)),
            source: Some(Rect::new(
                0.,
                offsets.top,
                offsets.left,
                texture.height() - offsets.top - offsets.bottom,
            )),
            ..Default::default()
        },
    );

    // middle
    draw_texture_ex(
        texture,
        rect.x + offsets.left,
        rect.y + offsets.top,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                rect.w - offsets.left - offsets.right,
                rect.h - offsets.top - offsets.bottom,
            )),
            source: Some(Rect::new(
                offsets.left,
                offsets.top,
                texture.width() - offsets.left - offsets.right,
                texture.height() - offsets.top - offsets.bottom,
            )),
            ..Default::default()
        },
    );

    // right
    draw_texture_ex(
        texture,
        rect.x + rect.w - offsets.left,
        rect.y + offsets.top,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(offsets.right, rect.h - offsets.top - offsets.bottom)),
            source: Some(Rect::new(
                texture.width() - offsets.left,
                offsets.top,
                offsets.right,
                texture.height() - offsets.top - offsets.bottom,
            )),
            ..Default::default()
        },
    );

    // bottom left
    draw_texture_ex(
        texture,
        rect.x,
        rect.y + rect.h - offsets.bottom,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                0.,
                texture.height() - offsets.top,
                offsets.left,
                offsets.top,
            )),
            ..Default::default()
        },
    );

    // bottom
    draw_texture_ex(
        texture,
        rect.x + offsets.left,
        rect.y + rect.h - offsets.bottom,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(rect.w - offsets.left - offsets.right, offsets.top)),
            source: Some(Rect::new(
                offsets.left,
                texture.height() - offsets.top,
                texture.width() - offsets.left - offsets.right,
                offsets.top,
            )),
            ..Default::default()
        },
    );

    // bottom right
    draw_texture_ex(
        texture,
        rect.x + rect.w - offsets.left,
        rect.y + rect.h - offsets.bottom,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                texture.width() - offsets.left,
                texture.height() - offsets.top,
                offsets.right,
                offsets.top,
            )),
            ..Default::default()
        },
    );
}
