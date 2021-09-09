pub use bevy_render::texture::*;

use crate::renderer::{RenderResource, RenderResourceType};
use bevy_asset::Handle;

impl RenderResource for Option<Handle<Texture>> {
    fn resource_type(&self) -> Option<RenderResourceType> {
        self.as_ref().map(|_texture| RenderResourceType::Texture)
    }

    fn write_buffer_bytes(&self, _buffer: &mut [u8]) {}

    fn buffer_byte_len(&self) -> Option<usize> {
        None
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        self.as_ref()
    }
}

impl RenderResource for Handle<Texture> {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Texture)
    }

    fn write_buffer_bytes(&self, _buffer: &mut [u8]) {}

    fn buffer_byte_len(&self) -> Option<usize> {
        None
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        Some(self)
    }
}
