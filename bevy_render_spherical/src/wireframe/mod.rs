use crate::{pipeline::PipelineDescriptor, shader::Shader};
use bevy_app::prelude::*;
use bevy_asset::Assets;
use bevy_ecs::system::IntoSystem;

mod pipeline;

pub use bevy_render::wireframe::{
    draw_wireframes_system, Wireframe, WireframeConfig, WIREFRAME_PIPELINE_HANDLE,
};

#[derive(Debug, Default)]
pub struct WireframePlugin;

impl Plugin for WireframePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<WireframeConfig>()
            .add_system_to_stage(crate::RenderStage::Draw, draw_wireframes_system.system());
        let world = app.world_mut().cell();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut pipelines = world
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        pipelines.set_untracked(
            WIREFRAME_PIPELINE_HANDLE,
            pipeline::build_wireframe_pipeline(&mut shaders),
        );
    }
}
