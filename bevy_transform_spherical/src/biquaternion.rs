use std::ops::*;

use bevy_math::{Mat4, Quat, Vec4};
use bevy_reflect::Reflect;

/// A pair of quaternions, representing a 4D rotation.
/// This is the rotation taking the vector `v`
/// to `Vec4::from(left * Quat::from(v) * right)`.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Biquaternion {
    pub left: Quat,
    pub right: Quat,
}

impl Biquaternion {
    pub const IDENTITY: Self = Self {
        left: Quat::IDENTITY,
        right: Quat::IDENTITY,
    };

    #[inline(always)]
    pub fn from_rotation(q: Quat) -> Self {
        Self {
            left: q,
            right: q.conjugate(),
        }
    }

    /// Returns the biquaternion conjugate of `self`. For a unit biquaternion the
    /// conjugate is also the inverse.
    #[inline(always)]
    pub fn conjugate(self) -> Self {
        Self {
            left: self.left.conjugate(),
            right: self.right.conjugate(),
        }
    }

    /// Returns the inverse of a normalized biquaternion.
    /// Typically biquaternion inverse returns the conjugate of a normalized biquaternion.
    /// Because `self` is assumed to already be unit length,
    /// this method does not normalize before returning the conjugate.
    #[inline(always)]
    pub fn inverse(self) -> Self {
        Self {
            left: self.left.inverse(),
            right: self.right.inverse(),
        }
    }

    #[inline(always)]
    pub fn lerp(self, end: Self, s: f32) -> Self {
        Self {
            left: self.left.lerp(end.left, s),
            right: self.right.lerp(end.right, s),
        }
    }

    #[inline(always)]
    pub fn slerp(self, end: Self, s: f32) -> Self {
        Self {
            left: self.left.slerp(end.left, s),
            right: self.right.slerp(end.right, s),
        }
    }

    /// Normalizes both quaternions in `self`.
    #[inline(always)]
    pub fn normalize(self) -> Self {
        Self {
            left: self.left.normalize(),
            right: self.right.normalize(),
        }
    }

    #[inline(always)]
    pub fn is_finite(self) -> bool {
        self.left.is_finite() && self.right.is_finite()
    }
    #[inline(always)]
    pub fn is_nan(self) -> bool {
        self.left.is_nan() || self.right.is_nan()
    }
    #[inline(always)]
    pub fn is_near_identity(self) -> bool {
        self.left.is_near_identity() && self.right.is_near_identity()
    }
    /// Returns whether both quaternions in `self` are normalized.
    #[inline(always)]
    pub fn is_normalized(self) -> bool {
        self.left.is_normalized() && self.right.is_normalized()
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    #[inline(always)]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: f32) -> bool {
        self.left.abs_diff_eq(other.left, max_abs_diff)
            && self.right.abs_diff_eq(other.right, max_abs_diff)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl From<Biquaternion> for Mat4 {
    fn from(bq: Biquaternion) -> Mat4 {
        Mat4::from_cols(bq * Vec4::X, bq * Vec4::Y, bq * Vec4::Z, bq * Vec4::W)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Default for Biquaternion {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add for Biquaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            left: self.left + rhs.left,
            right: self.right + rhs.right,
        }
    }
}

impl Sub for Biquaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            left: self.left - rhs.left,
            right: self.right - rhs.right,
        }
    }
}

impl Neg for Biquaternion {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            left: -self.left,
            right: -self.right,
        }
    }
}

impl Mul for Biquaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            left: self.left * rhs.left,
            right: rhs.right * self.right,
        }
    }
}

impl Mul<Vec4> for Biquaternion {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::from(self.left * Quat::from(rhs) * self.right)
    }
}

impl Mul<f32> for Biquaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            left: self.left * rhs,
            right: self.right * rhs,
        }
    }
}

impl Div<f32> for Biquaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            left: self.left / rhs,
            right: self.right / rhs,
        }
    }
}

impl MulAssign for Biquaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

// TODO: Remaining quaternion traits and methods.
// Decide whether they make sense here.

/*
Methods

from_rotation_arc
from_rotation_arc_colinear


// Maybe explicitly say from_rotation_around_xy vs from_rotation_in_xy?
from_rotation_x
from_rotation_y
from_rotation_z


Trait Implementations

Display
Sum<&'a Quat>
Product<&'a Quat>

*/

// TODO: Method docs
