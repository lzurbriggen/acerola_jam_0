use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

pub fn create_sprite_color_material() -> Material {
    let fragment_shader = FRAGMENT_SHADER.to_string();
    let vertex_shader = VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: &vertex_shader,
            fragment: &fragment_shader,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![("color".to_owned(), UniformType::Float4)],
            ..Default::default()
        },
    )
    .unwrap();

    material
}

const FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec4 color;

void main() {
    vec4 texture_color = texture2D(Texture, uv);
    if (texture_color.a <= 0.0) {
        discard;
    }
    gl_FragColor = texture_color * color;
}
";

const VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
