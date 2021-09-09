// #[cfg(feature = "hdr")]
// mod hdr_texture_loader {
//     pub use bevy_render::texture::HdrTextureLoader;
// }
// mod image_texture_loader {
//     pub use bevy_render::texture::{FileTextureError, ImageTextureLoader};
// }
#[cfg(feature = "hdr")]
mod hdr_texture_loader;
mod image_texture_loader;
mod sampler_descriptor {
    pub use bevy_render::texture::{
        AddressMode, FilterMode, SamplerBorderColor, SamplerDescriptor,
    };
}
#[allow(clippy::module_inception)]
mod texture;
// mod texture_descriptor {
//     pub use bevy_render::texture::{StorageTextureAccess, TextureDescriptor};
// }
mod texture_descriptor;
mod texture_dimension {
    pub use bevy_render::texture::{
        Extent3d, PixelInfo, TextureDimension, TextureFormat, TextureSampleType, TextureUsage,
        TextureViewDimension,
    };
}
mod image_texture_conversion;

#[cfg(feature = "hdr")]
pub use hdr_texture_loader::*;
pub use image_texture_loader::*;
pub use sampler_descriptor::*;
pub use texture::*;
pub use texture_descriptor::*;
pub use texture_dimension::*;

// pub use bevy_render::texture::*;
