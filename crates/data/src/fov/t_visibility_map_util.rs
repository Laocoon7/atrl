use crate::prelude::*;

/// A trait used by the fov algorithm to manipluate a visibility map.
pub trait TVisibilityMapUtility {
    /// gets visibility for a `Point2d`.
    fn get_visible(&self, p: impl Point2d) -> bool;

    /// sets visibility for a `Point2d`.
    fn set_visible(&mut self, p: impl Point2d);

    /// removes visibility for a `Point2d`.
    fn clear_visible(&mut self, p: impl Point2d);

    /// gets opacity for a `Point2d`.
    fn get_opaque(&self, p: impl Point2d) -> bool;

    /// sets opacity for a `Point2d`.
    fn set_opaque(&mut self, p: impl Point2d);

    /// removes opacity for a `Point2d`.
    fn clear_opaque(&mut self, p: impl Point2d);

    /// Clear visibility grid.
    fn clear_all(&mut self);

    // Clear anything marked visible.
    fn clear_all_visible(&mut self);
    
    // Clear anything marked opaque.
    fn clear_all_opaque(&mut self);
}
