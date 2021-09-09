#[allow(clippy::module_inception)]
mod shader;
mod shader_defs;

pub use shader::*;
pub use shader_defs::*;

pub use bevy_render::shader::{ShaderLayout, GL_FRONT_FACING, GL_INSTANCE_INDEX, GL_VERTEX_INDEX};
