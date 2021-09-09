use crate::{
    pipeline::{PipelineCompiler, PipelineDescriptor},
    renderer::RenderResourceContext,
};

use bevy_app::EventReader;
use bevy_asset::{AssetEvent, AssetLoader, Assets, LoadContext, LoadedAsset};
use bevy_ecs::system::{Res, ResMut};
use bevy_utils::{tracing::error, BoxedFuture};

pub use bevy_render::shader::{
    Shader, ShaderError, ShaderSource, ShaderStage, ShaderStages, ShaderStagesIterator,
};

#[cfg(not(target_arch = "wasm32"))]
pub use bevy_render::shader::glsl_to_spirv;

#[derive(Default)]
pub struct ShaderLoader;

impl AssetLoader for ShaderLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let shader = match ext {
                "vert" => Shader::from_glsl(ShaderStage::Vertex, std::str::from_utf8(bytes)?),
                "frag" => Shader::from_glsl(ShaderStage::Fragment, std::str::from_utf8(bytes)?),
                #[cfg(not(target_arch = "wasm32"))]
                "spv" => Shader::from_spirv(bytes)?,
                #[cfg(target_arch = "wasm32")]
                "spv" => panic!("cannot load .spv file on wasm"),
                _ => panic!("unhandled extension: {}", ext),
            };

            load_context.set_default_asset(LoadedAsset::new(shader));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vert", "frag", "spv"]
    }
}

pub fn shader_update_system(
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shader_events: EventReader<AssetEvent<Shader>>,
    mut pipeline_compiler: ResMut<PipelineCompiler>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
) {
    for event in shader_events.iter() {
        match event {
            AssetEvent::Modified { handle } => {
                if let Err(e) = pipeline_compiler.update_shader(
                    handle,
                    &mut pipelines,
                    &mut shaders,
                    &**render_resource_context,
                ) {
                    error!("Failed to update shader: {}", e);
                }
            }
            // Creating shaders on the fly is unhandled since they
            // have to exist already when assigned to a pipeline. If a
            // shader is removed the pipeline keeps using its
            // specialized version. Maybe this should be a warning?
            AssetEvent::Created { .. } | AssetEvent::Removed { .. } => (),
        }
    }
}
