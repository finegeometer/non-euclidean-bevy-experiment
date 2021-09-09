// mod bind_group {
//     pub use bevy_render::pipeline::{BindGroupDescriptor, BindGroupDescriptorId};
// }
// mod binding {
//     pub use bevy_render::pipeline::{BindType, BindingDescriptor, BindingShaderStage};
// }
mod bind_group;
mod binding;
#[allow(clippy::module_inception)]
mod pipeline;
mod pipeline_compiler;
mod pipeline_layout;
mod render_pipelines;
mod state_descriptors;
mod vertex_buffer_descriptor {
    pub use bevy_render::pipeline::{
        get_vertex_attribute_name_id, InputStepMode, VertexAttribute, VertexBufferLayout,
    };
}
mod vertex_format {
    pub use bevy_render::pipeline::{AsVertexFormats, VertexFormat};
}

pub use bind_group::*;
pub use binding::*;
pub use pipeline::*;
pub use pipeline_compiler::*;
pub use pipeline_layout::*;
pub use render_pipelines::*;
pub use state_descriptors::*;
pub use vertex_buffer_descriptor::*;
pub use vertex_format::*;
