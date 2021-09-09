use crate::{
    camera::{Camera, PerspectiveProjection, VisibleEntities},
    pipeline::RenderPipelines,
    prelude::Visible,
    render_graph::base,
    Draw, Mesh,
};
use base::MainPass;
use bevy_asset::Handle;
use bevy_ecs::bundle::Bundle;
use bevy_transform_spherical::components::{GlobalTransform, Transform};

/// A component bundle for "mesh" entities
#[derive(Bundle, Default)]
pub struct MeshBundle {
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Component bundle for camera entities with perspective projection
///
/// Use this for 3D rendering.
#[derive(Bundle)]
pub struct PerspectiveCameraBundle {
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl PerspectiveCameraBundle {
    pub fn new_3d() -> Self {
        Default::default()
    }

    pub fn with_name(name: &str) -> Self {
        PerspectiveCameraBundle {
            camera: Camera {
                name: Some(name.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl Default for PerspectiveCameraBundle {
    fn default() -> Self {
        PerspectiveCameraBundle {
            camera: Camera {
                name: Some(base::camera::CAMERA_3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
