use crate::prelude::*;

/// A trait used by the fov algorithm to calculate the resulting fov.
pub trait VisibilityProvider {
    /// Returns the `size` of the provided area.
    fn size(&self) -> UVec2;

    /// Returns true if the point is opaque.`
    fn is_opaque(&self, p: impl Point2d, vision_component: &Vision) -> bool;

    /// Returns the distance between two points.
    fn distance(&self, a: impl Point2d, b: impl Point2d) -> f32 {
        a.distance(b)
    }
}
