pub use bevy_render::camera::{CameraProjection, ScalingMode, WindowOrigin};

use super::DepthCalculation;
use bevy_ecs::reflect::ReflectComponent;
use bevy_math::{Mat4, Vec4};
use bevy_reflect::Reflect;

#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PerspectiveProjection {
    pub fov: f32,
    pub aspect_ratio: f32,
    /// The tangent of the distance to the near plane.
    pub tan_near: f32,
    /// The tangent of the distance to the far plane.
    pub tan_far: f32,
}

impl CameraProjection for PerspectiveProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        // A copy of `glam`'s `perspective_rh`, without the assert.
        fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
            let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
            let h = cos_fov / sin_fov;
            let w = h / aspect_ratio;
            let r = z_far / (z_near - z_far);
            Mat4::from_cols(
                Vec4::new(w, 0., 0., 0.),
                Vec4::new(0., h, 0., 0.),
                Vec4::new(0., 0., r, -1.),
                Vec4::new(0., 0., r * z_near, 0.),
            )
        }

        perspective_rh(self.fov, self.aspect_ratio, self.tan_near, self.tan_far)
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::Distance
    }
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        PerspectiveProjection {
            fov: std::f32::consts::PI / 4.0,
            tan_near: 0.01,
            tan_far: -0.01,
            aspect_ratio: 1.0,
        }
    }
}
