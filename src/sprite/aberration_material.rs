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
                ("color".to_owned(), UniformType::Float4),
                ("intensity".to_owned(), UniformType::Float1),
                ("time".to_owned(), UniformType::Float1),
            ],
            textures: vec!["noise1".to_owned(), "noise2".to_owned(), "mask".to_owned()],
            ..Default::default()
        },
    )
    .unwrap();

    material
}

const FRAGMENT_SHADER: &'static str = "#version 130
precision lowp float;

in vec2 uv;
out vec4 fragColor;

uniform sampler2D Texture;
uniform float intensity;
uniform float time;

uniform sampler2D noise1;
uniform sampler2D noise2;

void main() {
    vec2 texSize = vec2(textureSize(Texture, 0).xy);
    vec2 texCoord = gl_FragCoord.xy / texSize;

    float redOffset = 1.115 * intensity;
    float greenOffset = 1.112 * intensity;
    float blueOffset = -1.11 * intensity;

    // TODO: texture wrapping not available in macroquad?
    vec2 noise1Offset = fract(uv + vec2(time * 0.6));
    vec2 noise1Color = vec2(texture2D(noise1, noise1Offset));
    vec2 noise2Offset = fract(uv + vec2(time * 0.1));
    vec2 noise2Color = mix(noise1Color + vec2(texture2D(noise1, noise2Offset)), noise1Color, intensity);
    
    fragColor.r = texture2D(Texture, uv + vec2(noise2Color * redOffset) / texSize).r;
    fragColor.g = texture2D(Texture, uv + vec2(noise2Color * greenOffset) / texSize).g;
    fragColor.ba = texture2D(Texture, uv + vec2(noise2Color * blueOffset) / texSize).ba;
    // fragColor = texture2D(noise1, texCoord);
}
";

const VERTEX_SHADER: &'static str = "#version 130
precision lowp float;

in vec3 position;
in vec2 texcoord;

out vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
