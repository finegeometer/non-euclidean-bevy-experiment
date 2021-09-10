use bevy_core::Byteable;
use bevy_render_spherical::camera::{CameraProjection, PerspectiveProjection};
use bevy_transform_spherical::components::GlobalTransform;

pub use bevy_pbr::{AmbientLight, Light};

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
