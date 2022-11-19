use crate::prelude::*;

/// A trait used by the fov algorithm to manipluate a visibility map.
pub trait VisibilityMap {
    /// gets visibility for a `Point2d`.
    fn get_visible(&self, p: impl Point2d) -> bool;

    /// sets visibility for a `Point2d`.
    fn set_visible(&mut self, p: impl Point2d);
}
