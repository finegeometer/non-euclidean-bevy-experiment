use super::{Texture, TextureUsage};

pub use bevy_render::texture::{StorageTextureAccess, TextureDescriptor};

impl From<&Texture> for TextureDescriptor {
    fn from(texture: &Texture) -> Self {
        TextureDescriptor {
            size: texture.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: texture.dimension,
            format: texture.format,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        }
    }
}
