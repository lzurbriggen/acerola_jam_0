use macroquad::{
    material::{gl_use_default_material, gl_use_material, load_material, Material, MaterialParams},
    math::{vec2, Vec2},
    miniquad::{
        BlendFactor, BlendState, BlendValue, Equation, PipelineParams, ShaderSource, UniformType,
    },
};

use crate::{entity::entities::Ecs, game_data::GameData, sprite::indexed_sprite::IndexedSprite};

pub struct HudHearts {
    sprite: IndexedSprite,
}

impl HudHearts {
    pub fn new(data: &GameData) -> Self {
        Self {
            sprite: IndexedSprite::new(data, "hud_heart", 16, Vec2::ZERO),
        }
    }

    pub fn draw(&self, data: &GameData, ecs: &Ecs) {
        let players = ecs.check_components(|e, comps| {
            comps.player_data.contains_key(e) && comps.health.contains_key(e)
        });

        let start_pos = vec2(16., 0.);
        for player_e in players {
            let player = ecs.components.player_data.get(&player_e).unwrap();
            let health = ecs.components.health.get(&player_e).unwrap();

            for i in 0..player.max_hp {
                let heart_index = if (i as f32) < health.hp { 0 } else { 2 };
                self.sprite.draw(
                    data,
                    start_pos.x + (vec2(i as f32 * 16., start_pos.y)),
                    heart_index,
                    false,
                )
            }
        }
    }
}

pub struct AberrationMeter {
    sprite: IndexedSprite,
}

impl AberrationMeter {
    pub fn new(data: &GameData) -> Self {
        Self {
            sprite: IndexedSprite::new(data, "aberration_meter", 48, Vec2::ZERO),
        }
    }

    pub fn draw(&self, data: &GameData, ecs: &Ecs) {
        let players = ecs.check_components(|e, comps| comps.player_data.contains_key(e));

        let pos = vec2(308., 78.);
        for player_e in players {
            let player = ecs.components.player_data.get(&player_e).unwrap();

            gl_use_material(&data.graphics.aberration_meter_material);

            let h = player.aberration * 65.;
            // TODO: texture instead so material applies properly
            // draw_rectangle_ex(
            //     pos.x + 9.,
            //     pos.y + 16. + 65. - h,
            //     30.,
            //     h,
            //     DrawRectangleParams {
            //         color: Color::from_hex(0x793a80),
            //         ..Default::default()
            //     },
            // );
            // data.sprites
            //     .aberration_material
            //     .set_uniform("enable_mask", 1);
            data.graphics
                .aberration_meter_material
                .set_uniform("cutoff", player.aberration);
            // data.sprites.aberration_meter.draw_with_dest(
            //     vec2(pos.x + 9., pos.y + 16. + 65. - h),
            //     3,
            //     Some(vec2(30., h)),
            // );
            // data.sprites.aberration_meter.draw_with_dest(
            //     vec2(pos.x + 9., pos.y + 16. + 65. - h),
            //     3,
            //     Some(vec2(30., h)),
            // );
            self.sprite.draw(data, pos, 3, false);

            data.graphics
                .aberration_meter_material
                .set_uniform("cutoff", 1f32);
            // data.sprites
            //     .aberration_material
            //     .set_uniform("enable_mask", 0);
            self.sprite.draw(data, pos, 0, false);
            // data.sprites
            //     .aberration_material
            //     .set_uniform("enable_mask", 1);
            self.sprite.draw(
                data,
                pos + vec2(0., (1. - player.aberration) * 65.),
                2,
                false,
            );
            // data.sprites
            //     .aberration_material
            //     .set_uniform("enable_mask", 0);

            self.sprite.draw(data, pos, 1, false);

            gl_use_default_material();
        }
    }
}

pub fn create_aberration_meter_material() -> Material {
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
                ("intensity".to_owned(), UniformType::Float1),
                ("time".to_owned(), UniformType::Float1),
                ("cutoff".to_owned(), UniformType::Float1),
                ("enable_mask".to_owned(), UniformType::Int1),
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
uniform float cutoff;
uniform bool enable_mask;

uniform sampler2D noise1;
uniform sampler2D noise2;
uniform sampler2D mask;

void main() {
    float redOffset   =  0.015 * intensity;
    float greenOffset =  0.012 * intensity;
    float blueOffset  =  0.01 * intensity;

    vec4 texture_color = texture2D(Texture, uv);
    vec2 texSize = vec2(textureSize(Texture, 0).xy);
    // vec2 texCoord = gl_FragCoord.xy / texSize;

    // TODO: texture wrapping not available in macroquad?
    vec2 texSize2 = vec2(textureSize(noise2, 0).xy);
    vec2 noise1Offset = fract(uv + vec2(time * 0.6));
    vec2 noise1Color = vec2(texture2D(noise1, noise1Offset));
    vec2 noise2Offset = fract(uv + vec2(time * 0.1));
    vec2 noise2Color = mix(noise1Color + vec2(texture2D(noise1, noise2Offset)), noise1Color, intensity);
    if (enable_mask && texture2D(mask, uv + noise2Color).r < 1) {
        noise2Color.rg = vec2(0);
    }
    fragColor.r  = texture2D(Texture, uv + (noise2Color * intensity * vec2(redOffset))).r;
    fragColor.g  = texture2D(Texture, uv + (noise2Color * intensity * vec2(greenOffset))).g;
    fragColor.ba = texture2D(Texture, uv + (noise2Color * intensity * vec2(blueOffset))).ba;
    // fragColor = texture2D(Texture, uv);
    // fragColor = vec4(noise2Color.r);
    // fragColor = texture2D(mask, uv);
    if (cutoff < 1 && uv.y < 1. - (cutoff * 64. + 16.) / 96.) {
        fragColor.a = 0;

    }
    if (enable_mask && texture2D(mask, uv + noise2Color).r < 1) {
        fragColor.a = 0;
    }
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
