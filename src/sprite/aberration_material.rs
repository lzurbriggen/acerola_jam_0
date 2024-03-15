use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

pub fn create_aberration_material() -> Material {
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
            uniforms: vec![
                ("texture_size".to_owned(), UniformType::Float2),
                ("color".to_owned(), UniformType::Float4),
                ("intensity".to_owned(), UniformType::Float1),
                ("time".to_owned(), UniformType::Float1),
                ("hue_shift".to_owned(), UniformType::Float1),
            ],
            textures: vec!["noise1".to_owned(), "noise2".to_owned(), "mask".to_owned()],
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
uniform vec2 texture_size;
uniform float intensity;
uniform float time;
uniform float hue_shift;

uniform sampler2D noise1;
uniform sampler2D noise2;


vec3 hueShift(vec3 color, float hue) {
    const vec3 k = vec3(0.57735, 0.57735, 0.57735);
    float cosAngle = cos(hue);
    return vec3(color * cosAngle + cross(k, color) * sin(hue) + k * dot(k, color) * (1.0 - cosAngle));
}

void main() {
    float redOffset = 2.115 * intensity;
    float greenOffset = 2.112 * intensity;
    float blueOffset = -2.11 * intensity;

    // TODO: texture wrapping not available in macroquad?
    vec2 noise1Offset = fract(uv + vec2(time * 0.01 * intensity));
    vec2 noise1Color = vec2(texture2D(noise1, noise1Offset));
    vec2 noise2Offset = fract(uv + vec2(time * 0.001 * intensity));
    vec2 noise2Color = mix(noise1Color + vec2(texture2D(noise1, noise2Offset)), noise1Color, 0.5);
    
    gl_FragColor.r = texture2D(Texture, uv + vec2(noise2Color * redOffset) / texture_size).r;
    gl_FragColor.g = texture2D(Texture, uv + vec2(noise2Color * greenOffset) / texture_size).g;
    gl_FragColor.ba = texture2D(Texture, uv + vec2(noise2Color * blueOffset) / texture_size).ba;
    gl_FragColor.rgb = hueShift(gl_FragColor.rgb, hue_shift);
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
