#[allow(clippy::module_inception)]
mod render_resource;
mod shared_buffers;

pub use bevy_render::renderer::{
    AssetRenderResourceBindings, BindGroup, BindGroupBuilder, BindGroupId, BindGroupStatus,
    BufferId, BufferInfo, BufferMapMode, BufferUsage, IndexedBindGroupEntry, RenderResourceBinding,
    RenderResourceBindings, SamplerId, TextureId,
};

pub use render_resource::*;
pub use shared_buffers::*;
