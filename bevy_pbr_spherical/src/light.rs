use bevy_core::Byteable;
use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;
use bevy_render::{
    camera::{CameraProjection, PerspectiveProjection},
    color::Color,
};
use bevy_transform::components::GlobalTransform;
use std::ops::Range;

/// A point light
#[derive(Debug, Reflect)]
#[reflect(Component)]
pub struct Light {
    pub color: Color,
    pub fov: f32,
    pub depth: Range<f32>,
    pub intensity: f32,
    pub range: f32,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            color: Color::rgb(1.0, 1.0, 1.0),
            depth: 0.1..1.57,
            fov: f32::to_radians(60.0),
            intensity: 200.0,
            range: 20.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct LightRaw {
    pub proj: [[f32; 4]; 4],
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

unsafe impl Byteable for LightRaw {}

impl LightRaw {
    pub fn from(light: &Light, global_transform: &GlobalTransform) -> LightRaw {
        let perspective = PerspectiveProjection {
            fov: light.fov,
            aspect_ratio: 1.0,
            tan_near: light
                .depth
                .start
                .clamp(0.00001, std::f32::consts::PI - 0.00001)
                .tan(),
            tan_far: light
                .depth
                .end
                .clamp(0.00001, std::f32::consts::PI - 0.00001)
                .tan(),
        };

        let proj = perspective.get_projection_matrix() * global_transform.compute_matrix();

        // premultiply color by intensity
        // we don't use the alpha at all, so no reason to multiply only [0..3]
        let color: [f32; 4] = (light.color * light.intensity).into();
        LightRaw {
            proj: proj.to_cols_array_2d(),
            pos: (global_transform.position() / light.range).into(), // dot(pos,pos) is the attenuation.
            color,
        }
    }
}

// Ambient light color.
#[derive(Debug)]
pub struct AmbientLight {
    pub color: Color,
    /// Color is premultiplied by brightness before being passed to the shader
    pub brightness: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::rgb(1.0, 1.0, 1.0),
            brightness: 0.05,
        }
    }
}
