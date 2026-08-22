//! Compatibility shims for transform precision.
//!
//! On the `transform_f64` Bevy fork, `bevy::math` exposes `TVec3`/`TQuat` (transform-precision
//! vector/quaternion types) and the `ToPrecision`/`ToRender` conversion traits. Upstream Bevy
//! keeps `Transform` at `f32` precision, so these resolve to the plain `f32` types and the
//! conversions are (near-)identities.

use bevy::math::{Quat, Vec3};

/// Transform-precision 3D vector. Resolves to [`Vec3`] on upstream Bevy.
pub type TVec3 = Vec3;
/// Transform-precision quaternion. Resolves to [`Quat`] on upstream Bevy.
pub type TQuat = Quat;

/// Converts a value to transform precision (`f32` on upstream Bevy).
pub trait ToPrecision {
    /// The transform-precision counterpart of this type.
    type Precision;
    /// Converts `self` to transform precision.
    fn to_precision(self) -> Self::Precision;
}

impl ToPrecision for f32 {
    type Precision = f32;
    #[inline]
    fn to_precision(self) -> f32 {
        self
    }
}

impl ToPrecision for f64 {
    type Precision = f32;
    #[inline]
    fn to_precision(self) -> f32 {
        self as f32
    }
}

impl ToPrecision for Vec3 {
    type Precision = TVec3;
    #[inline]
    fn to_precision(self) -> TVec3 {
        self
    }
}

impl ToPrecision for Quat {
    type Precision = TQuat;
    #[inline]
    fn to_precision(self) -> TQuat {
        self
    }
}

/// Converts a transform-precision value to render precision (`f32` on upstream Bevy).
pub trait ToRender {
    /// The render-precision counterpart of this type.
    type Render;
    /// Converts `self` to render precision.
    fn to_render(self) -> Self::Render;
}

impl ToRender for f32 {
    type Render = f32;
    #[inline]
    fn to_render(self) -> f32 {
        self
    }
}

impl ToRender for Vec3 {
    type Render = Vec3;
    #[inline]
    fn to_render(self) -> Vec3 {
        self
    }
}

impl ToRender for Quat {
    type Render = Quat;
    #[inline]
    fn to_render(self) -> Quat {
        self
    }
}
