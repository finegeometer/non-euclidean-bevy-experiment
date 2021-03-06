pub use bevy_pbr::render_graph::{node, uniform};

mod lights_node;
mod pbr_pipeline;

use bevy_ecs::world::World;
pub use lights_node::*;
pub use pbr_pipeline::*;

use crate::prelude::StandardMaterial;
use bevy_asset::Assets;
use bevy_render_spherical::{
    pipeline::PipelineDescriptor,
    render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    shader::Shader,
};
use bevy_transform_spherical::prelude::GlobalTransform;

pub(crate) fn add_pbr_graph(world: &mut World) {
    {
        let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_system_node(
            node::TRANSFORM,
            RenderResourcesNode::<GlobalTransform>::new(true),
        );
        graph.add_system_node(
            node::STANDARD_MATERIAL,
            AssetRenderResourcesNode::<StandardMaterial>::new(true),
        );
        graph.add_system_node(node::LIGHTS, LightsNode::new(10));

        // TODO: replace these with "autowire" groups
        graph
            .add_node_edge(node::STANDARD_MATERIAL, base::node::MAIN_PASS)
            .unwrap();
        graph
            .add_node_edge(node::TRANSFORM, base::node::MAIN_PASS)
            .unwrap();
        graph
            .add_node_edge(node::LIGHTS, base::node::MAIN_PASS)
            .unwrap();
    }
    let pipeline = build_pbr_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());
    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();
    pipelines.set_untracked(PBR_PIPELINE_HANDLE, pipeline);
}
