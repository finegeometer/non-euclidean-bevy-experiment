#[allow(clippy::module_inception)]
mod shared_buffers;

pub use bevy_render::renderer::{
    AssetRenderResourceBindings, BindGroup, BindGroupBuilder, BindGroupId, BindGroupStatus,
    BufferId, BufferInfo, BufferMapMode, BufferUsage, IndexedBindGroupEntry, RenderResource,
    RenderResourceBinding, RenderResourceBindings, RenderResourceHints, RenderResourceId,
    RenderResourceIterator, RenderResourceType, RenderResources, SamplerId, TextureId,
};

pub use shared_buffers::*;
