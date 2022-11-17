use crate::prelude::*;

/// A trait used by the fov algorithm to calculate the resulting fov.
pub trait VisibilityMap {
    /// Returns true if the point is opaque.`
    fn is_opaque(&self, p: impl Point2d) -> bool;
    /// Returns true if the point is in bounds.
    fn is_in_bounds(&self, p: impl Point2d) -> bool;
    /// Set the visibility of a point.
    fn set_visible(&mut self, p: impl Point2d);
    /// Returns the distance between two points.
    fn distance(&self, a: impl Point2d, b: impl Point2d) -> f32;
}

impl VisibilityMap for VisibilityMap2d {
    fn is_opaque(&self, p: impl Point2d) -> bool {
        if self.in_bounds(p) {
            self[p].opaque
        } else {
            true
        }
    }

    fn is_in_bounds(&self, p: impl Point2d) -> bool {
        self.in_bounds(p)
    }

    fn set_visible(&mut self, p: impl Point2d) {
        if self.in_bounds(p) {
            self[p].visible = true;
        }
    }

    fn distance(&self, a: impl Point2d, b: impl Point2d) -> f32 {
        a.distance(b)
    }
}

/// A trait used by the fov algorithm to manipluate a visibility map.
pub trait VisibilityMapUtility {
    /// Clear all opaque tiles from the map
    fn clear_opaque(&mut self);
    /// Clear all visible tiles from the map
    fn clear_visible(&mut self);
    /// toggle visibility for a `Point2d`.
    fn toggle_opaque(&mut self, p: impl Point2d);
    /// toggle visibility for a `Point2d`.
    fn toggle_visible(&mut self, p: impl Point2d);
}

impl VisibilityMapUtility for VisibilityMap2d {
    fn toggle_opaque(&mut self, p: impl Point2d) {
        let i = self.get_idx_unchecked(p);
        self[i].opaque = !self[i].opaque;
    }

    fn toggle_visible(&mut self, p: impl Point2d) {
        let i = self.get_idx_unchecked(p);
        self[i].visible = !self[i].visible;
    }

    fn clear_opaque(&mut self) {
        self.iter_mut().for_each(|p| p.opaque = false);
    }

    fn clear_visible(&mut self) {
        self.iter_mut().for_each(|p| p.visible = false);
    }
}
