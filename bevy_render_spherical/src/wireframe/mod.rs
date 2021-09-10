use crate::{
    draw::DrawContext,
    mesh::Indices,
    pipeline::{PipelineDescriptor, PipelineSpecialization, RenderPipeline},
    prelude::*,
    shader::Shader,
};
use bevy_app::prelude::*;
use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    query::With,
    system::{IntoSystem, Query, QuerySet, Res},
    world::Mut,
};
use bevy_utils::HashSet;

mod pipeline;

pub use bevy_render::wireframe::{Wireframe, WireframeConfig, WIREFRAME_PIPELINE_HANDLE};

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

pub fn draw_wireframes_system(
    mut draw_context: DrawContext,
    msaa: Res<Msaa>,
    meshes: Res<Assets<Mesh>>,
    wireframe_config: Res<WireframeConfig>,
    mut query: QuerySet<(
        Query<(&mut Draw, &mut RenderPipelines, &Handle<Mesh>, &Visible)>,
        Query<(&mut Draw, &mut RenderPipelines, &Handle<Mesh>, &Visible), With<Wireframe>>,
    )>,
) {
    let iterator = |(mut draw, mut render_pipelines, mesh_handle, visible): (
        Mut<Draw>,
        Mut<RenderPipelines>,
        &Handle<Mesh>,
        &Visible,
    )| {
        if !visible.is_visible {
            return;
        }

        // don't render if the mesh isn't loaded yet
        let mesh = if let Some(mesh) = meshes.get(mesh_handle) {
            mesh
        } else {
            return;
        };

        let mut render_pipeline = RenderPipeline::specialized(
            WIREFRAME_PIPELINE_HANDLE.typed(),
            PipelineSpecialization {
                sample_count: msaa.samples,
                strip_index_format: None,
                shader_specialization: Default::default(),
                primitive_topology: mesh.primitive_topology(),
                dynamic_bindings: render_pipelines
                    .bindings
                    .iter_dynamic_bindings()
                    .map(|name| name.to_string())
                    .collect::<HashSet<String>>(),
                vertex_buffer_layout: mesh.get_vertex_buffer_layout(),
            },
        );
        render_pipeline.dynamic_bindings_generation =
            render_pipelines.bindings.dynamic_bindings_generation();

        draw_context
            .set_pipeline(
                &mut draw,
                &render_pipeline.pipeline,
                &render_pipeline.specialization,
            )
            .unwrap();
        draw_context
            .set_bind_groups_from_bindings(&mut draw, &mut [&mut render_pipelines.bindings])
            .unwrap();
        draw_context
            .set_vertex_buffers_from_bindings(&mut draw, &[&render_pipelines.bindings])
            .unwrap();

        match mesh.indices() {
            Some(Indices::U32(indices)) => draw.draw_indexed(0..indices.len() as u32, 0, 0..1),
            Some(Indices::U16(indices)) => draw.draw_indexed(0..indices.len() as u32, 0, 0..1),
            None => draw.draw(0..mesh.count_vertices() as u32, 0..1),
        };
    };

    if wireframe_config.global {
        query.q0_mut().iter_mut().for_each(iterator);
    } else {
        query.q1_mut().iter_mut().for_each(iterator);
    }
}
