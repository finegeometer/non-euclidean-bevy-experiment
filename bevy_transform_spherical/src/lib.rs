pub mod biquaternion;
pub mod components;
pub use ::bevy_transform::hierarchy;
pub mod transform_propagate_system;
pub use bevy_transform::TransformSystem;

pub mod prelude {
    pub use crate::{components::*, hierarchy::*, TransformPlugin};
}

use bevy_app::prelude::*;
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::IntoSystem};
use prelude::{parent_update_system, Children, GlobalTransform, Parent, PreviousParent, Transform};

#[derive(Default)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<Children>()
            .register_type::<Parent>()
            .register_type::<PreviousParent>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            // add transform systems to startup so the first update is "correct"
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                parent_update_system
                    .system()
                    .label(TransformSystem::ParentUpdate),
            )
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                transform_propagate_system::transform_propagate_system
                    .system()
                    .label(TransformSystem::TransformPropagate)
                    .after(TransformSystem::ParentUpdate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                parent_update_system
                    .system()
                    .label(TransformSystem::ParentUpdate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                transform_propagate_system::transform_propagate_system
                    .system()
                    .label(TransformSystem::TransformPropagate)
                    .after(TransformSystem::ParentUpdate),
            );
    }
}
