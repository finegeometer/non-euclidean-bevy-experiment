use super::Transform;
use bevy_ecs::reflect::ReflectComponent;
use bevy_math::{Mat4, Quat, Vec3, Vec4};
use bevy_reflect::Reflect;
use std::ops::Mul;

use crate::biquaternion::Biquaternion;

/// Describe the position of an entity relative to the reference frame.
///
/// * To place or move an entity, you should set its [`Transform`].
/// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
/// * To get the global position of an entity, you should get its [`GlobalTransform`].
///
/// ## [`Transform`] and [`GlobalTransform`]
///
/// [`Transform`] is the position of an entity relative to its parent position, or the reference
/// frame if it doesn't have a [`Parent`](super::Parent).
///
/// [`GlobalTransform`] is the position of an entity relative to the reference frame.
///
/// [`GlobalTransform`] is updated from [`Transform`] in the system
/// [`transform_propagate_system`](crate::transform_propagate_system::transform_propagate_system).
///
/// In pseudo code:
/// ```ignore
/// for entity in entities_without_parent:
///     set entity.global_transform to entity.transform
///     recursively:
///         set parent to current entity
///         for child in parent.children:
///             set child.global_transform to parent.global_transform * child.transform
/// ```
///
/// This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
/// update the[`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
/// before the [`GlobalTransform`] is updated.
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component, PartialEq)]
pub struct GlobalTransform {
    pub biquat: Biquaternion,
}

impl GlobalTransform {
    /// Creates a new identity [`GlobalTransform`], with no translation, rotation, and a scale of 1
    /// on all axes.
    #[inline]
    pub const fn identity() -> Self {
        GlobalTransform {
            biquat: Biquaternion::IDENTITY,
        }
    }

    pub fn position(&self) -> Vec4 {
        self.biquat * Vec4::W
    }

    //     #[doc(hidden)]
    //     #[inline]
    //     pub fn from_matrix(matrix: Mat4) -> Self {
    //         let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

    //         GlobalTransform {
    //             translation,
    //             rotation,
    //             scale,
    //         }
    //     }

    #[doc(hidden)]
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Transform::from_translation(translation).into()
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_small_translation(translation: Vec3) -> Self {
        Transform::from_small_translation(translation).into()
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Transform::from_rotation(rotation).into()
    }

    //     #[doc(hidden)]
    //     #[inline]
    //     pub fn looking_at(mut self, target: Vec3, up: Vec3) -> Self {
    //         self.look_at(target, up);
    //         self
    //     }

    /// Returns the 3d affine transformation matrix from this transforms translation,
    /// rotation, and scale.
    #[inline]
    pub fn compute_matrix(&self) -> Mat4 {
        Mat4::from(self.biquat)
    }

    //     #[doc(hidden)]
    //     #[inline]
    //     pub fn rotate(&mut self, rotation: Quat) {
    //         self.rotation *= rotation;
    //     }

    /// Returns the composite [`GlobalTransform`] resulting from applying `transform`, followed by `self`.
    #[inline]
    pub fn mul_transform(&self, transform: Transform) -> GlobalTransform {
        Self {
            biquat: self.biquat * transform.biquat,
        }
    }

    /// Returns a [`Vec4`] of this [`Transform`] applied to `value`.
    #[inline]
    pub fn mul_vec4(&self, value: Vec4) -> Vec4 {
        self.biquat * value
    }

    //     #[doc(hidden)]
    //     #[inline]
    //     pub fn look_at(&mut self, target: Vec3, up: Vec3) {
    //         let forward = Vec3::normalize(self.translation - target);
    //         let right = up.cross(forward).normalize();
    //         let up = forward.cross(right);
    //         self.rotation = Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward));
    //     }
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Transform> for GlobalTransform {
    fn from(transform: Transform) -> Self {
        Self {
            biquat: transform.biquat,
        }
    }
}

// impl Mul<GlobalTransform> for GlobalTransform {
//     type Output = GlobalTransform;

//     #[inline]
//     fn mul(self, global_transform: GlobalTransform) -> Self::Output {
//         self.mul_transform(global_transform.into())
//     }
// }

impl Mul<Transform> for GlobalTransform {
    type Output = GlobalTransform;

    #[inline]
    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<Vec4> for GlobalTransform {
    type Output = Vec4;

    #[inline]
    fn mul(self, value: Vec4) -> Self::Output {
        self.mul_vec4(value)
    }
}

// TODO: Uncomment

// TODO: I don't understand the multiplication order for GlobalTransform.
// When implementing the system, figure this out!

#[cfg(feature = "render")]
mod render {
    use super::GlobalTransform;
    use bevy_asset::Handle;
    use bevy_core::Bytes;
    use bevy_render::{
        prelude::Texture,
        renderer::{RenderResource, RenderResourceIterator, RenderResourceType, RenderResources},
    };

    impl RenderResource for GlobalTransform {
        fn resource_type(&self) -> Option<RenderResourceType> {
            Some(RenderResourceType::Buffer)
        }

        fn write_buffer_bytes(&self, buffer: &mut [u8]) {
            let mat4 = self.compute_matrix();
            mat4.write_bytes(buffer);
        }

        fn buffer_byte_len(&self) -> Option<usize> {
            Some(std::mem::size_of::<[f32; 16]>())
        }

        fn texture(&self) -> Option<&Handle<Texture>> {
            None
        }
    }

    impl RenderResources for GlobalTransform {
        fn render_resources_len(&self) -> usize {
            1
        }

        fn get_render_resource(&self, index: usize) -> Option<&dyn RenderResource> {
            if index == 0 {
                Some(self)
            } else {
                None
            }
        }

        fn get_render_resource_name(&self, index: usize) -> Option<&str> {
            if index == 0 {
                Some("Transform")
            } else {
                None
            }
        }

        fn iter(&self) -> RenderResourceIterator {
            RenderResourceIterator::new(self)
        }
    }
}
