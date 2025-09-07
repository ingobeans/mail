use std::sync::{LazyLock, Mutex};

use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

pub const SCREEN_WIDTH: f32 = 256.0;
pub const SCREEN_HEIGHT: f32 = 144.0;

pub const MAX_VELOCITY: f32 = 1.2;
pub const GROUND_FRICTION: f32 = 0.21;
pub const AIR_DRAG: f32 = 0.07;
pub const GRAVITY: f32 = 0.9;

pub static IS_DEBUG: Mutex<bool> = Mutex::new(false);

pub fn create_camera(w: f32, h: f32) -> Camera2D {
    let rt = render_target(w as u32, h as u32);
    rt.texture.set_filter(FilterMode::Nearest);
    
    Camera2D {
        render_target: Some(rt),
        zoom: Vec2::new(1.0 / w * 2.0, 1.0 / h * 2.0),
        ..Default::default()
    }
}

pub static COLORS: &[Vec4] = &[
    Vec4::new(1.0, 1.0, 1.0, 1.0),
    Vec4::new(0.0, 0.0, 0.0, 1.0),
    Color::from_hex(0xda2424).to_vec(),
    Color::from_hex(0x2890dc).to_vec(),
    Color::from_hex(0x08b23b).to_vec(),
    Color::from_hex(0x720d0d).to_vec(),
];

pub static COLOR_MOD_MATERIAL: LazyLock<Material> = LazyLock::new(|| {
    // to enable transparency!
    let pipeline = PipelineParams {
        alpha_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    };
    let m = load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: COLOR_MOD_FRAGMENT,
        },
        MaterialParams {
            pipeline_params: pipeline,
            uniforms: vec![UniformDesc::new("color", UniformType::Float4)],
            ..Default::default()
        },
    )
    .unwrap();
    m.set_uniform("color", COLORS[0]);
    m
});

pub const COLOR_MOD_FRAGMENT: &str = "#version 100
precision lowp float;

varying vec2 uv;

uniform lowp vec4 color;

uniform sampler2D Texture;

void main() {
    if (texture2D(Texture, uv).a > 0.0) {
        gl_FragColor = color;
    } else {
        gl_FragColor = texture2D(Texture, uv);
    }
}
";

pub const DEFAULT_VERTEX_SHADER: &str = "#version 100
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
