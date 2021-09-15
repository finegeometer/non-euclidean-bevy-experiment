use super::GlobalTransform;
use bevy_ecs::reflect::ReflectComponent;
use bevy_math::{Mat3, Quat, Vec3, Vec4};
use bevy_reflect::Reflect;
use std::ops::Mul;

use crate::biquaternion::Biquaternion;

/// Describe the position of an entity. If the entity has a parent, the position is relative
/// to its parent position.
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
pub struct Transform {
    pub biquat: Biquaternion,
}

impl Transform {
    /// Creates a new identity [`Transform`], with no translation or rotation.
    #[inline]
    pub const fn identity() -> Self {
        Transform {
            biquat: Biquaternion::IDENTITY,
        }
    }

    /// Creates a new [`Transform`], that translates by the given vector.
    /// If the vector is large, the translation will curve around the sphere.
    /// For example, translating by distance 2π will be the identity,
    /// because you go all the way around the sphere.
    #[inline]
    pub fn from_translation(mut translation: Vec3) -> Self {
        translation *= 0.5;

        let len = translation.length();
        let (sin_len_by_len, cos_len) = if len < 0.0001 {
            (1., 1.)
        } else {
            let (s, c) = len.sin_cos();
            (s / len, c)
        };

        translation *= sin_len_by_len;

        let quat = Quat::from_xyzw(translation.x, translation.y, translation.z, cos_len);
        Transform {
            biquat: Biquaternion {
                left: quat,
                right: quat,
            },
        }
    }

    /// Creates a new [`Transform`], that translates by the given vector.
    /// Is only accurate when the length of `translation` is much smaller than one,
    /// which is the size of the spherical universe.
    #[inline]
    pub fn from_small_translation(mut translation: Vec3) -> Self {
        translation *= 0.5;
        let quat = Quat::from_xyzw(translation.x, translation.y, translation.z, 1.).normalize();
        Transform {
            biquat: Biquaternion {
                left: quat,
                right: quat,
            },
        }
    }

    /// Creates a new [`Transform`], that rotates by the given quaternion.
    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            biquat: Biquaternion::from_rotation(rotation),
        }
    }

    /// Updates and returns this [`Transform`] by rotating it so that its unit vector in the
    /// local z direction is toward `target` and its unit vector in the local y direction
    /// is toward `up`.
    #[inline]
    pub fn looking_at(mut self, target: Vec4, up: Vec4) -> Self {
        self.look_at(target, up);
        self
    }

    //     /// Returns the 3d affine transformation matrix from this transforms translation,
    //     /// rotation, and scale.
    //     #[inline]
    //     pub fn compute_matrix(&self) -> Mat4 {
    //         Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    //     }

    //     /// Rotates the transform by the given rotation.
    //     #[inline]
    //     pub fn rotate(&mut self, rotation: Quat) {
    //         self.rotation *= rotation;
    //     }

    /// Returns the composite [`Transform`] resulting from applying `transform`, followed by `self`.
    #[inline]
    pub fn mul_transform(&self, transform: Transform) -> Self {
        Transform {
            biquat: self.biquat * transform.biquat,
        }
    }

    /// Returns a [`Vec4`] of this [`Transform`] applied to `value`.
    #[inline]
    pub fn mul_vec4(&self, value: Vec4) -> Vec4 {
        self.biquat * value
    }

    /// Rotates this [`Transform`] so that its unit vector in the local z direction is toward
    /// `target` and its unit vector in the local y direction is toward `up`.
    #[inline]
    pub fn look_at(&mut self, target: Vec4, up: Vec4) {
        // Convert from world space to body space.
        let inv = self.biquat.inverse();
        let forward: Vec3 = (inv * -target).into();
        let up: Vec3 = (inv * up).into();

        // Calculate the rotation, in body space.
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        let rotation = Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward));

        self.biquat *= Biquaternion::from_rotation(rotation);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<GlobalTransform> for Transform {
    fn from(transform: GlobalTransform) -> Self {
        Self {
            biquat: transform.biquat,
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<Vec4> for Transform {
    type Output = Vec4;

    fn mul(self, value: Vec4) -> Self::Output {
        self.mul_vec4(value)
    }
}

// TODO: Uncomment
// TODO: Should I have a separate `Position` type?
// Argument in favor: will probably want that in hyperbolic case, because rounding error.
// But where should I put the module?
