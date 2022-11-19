use crate::prelude::*;
use bitflags::bitflags;

bitflags! {
    #[derive(Reflect, FromReflect)]
    pub struct VisibilityFlag: u32 {
        const NONE = 0b00000000;
        const VISIBLE = 0b00000001;
        const OPAQUE = 0b00000010;
        const VISIBLE_AND_OPAQUE = Self::VISIBLE.bits | Self::OPAQUE.bits;
    }
}

// explicit `Default` implementation
impl Default for VisibilityFlag {
    fn default() -> Self {
        Self::NONE
    }
}

impl Default for &VisibilityFlag {
    fn default() -> &'static VisibilityFlag {
        &VisibilityFlag::NONE
    }
}

pub type VisibilityMap2d = Grid<VisibilityFlag>;

/// A trait used by the fov algorithm to calculate the resulting fov.
pub trait VisibilityMap {
    /// Returns true if the point is opaque.`
    fn is_opaque(&self, p: impl Point2d) -> bool;

    /// Returns true if the point is in bounds.
    fn is_in_bounds(&self, p: impl Point2d) -> bool;

    /// Set the visibility of a point.
    fn set_visible(&mut self, p: impl Point2d);

    /// Returns the distance between two points.
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
    /// Clear visibility grid from `opaque` and `visible` tiles.
    fn clear_all(&mut self);
}

impl VisibilityMap2d {
    pub fn get_visibility(&self, p: impl Point2d) -> bool {
        self.get(p).unwrap().contains(VisibilityFlag::VISIBLE)
    }

    pub fn get_opaque(&self, p: impl Point2d) -> bool {
        self.get(p).unwrap_or_default().contains(VisibilityFlag::OPAQUE)
    }
}

impl VisibilityMap for VisibilityMap2d {
    fn is_opaque(&self, p: impl Point2d) -> bool {
        if self.in_bounds(p) {
            self[p].contains(VisibilityFlag::OPAQUE)
        } else {
            true
        }
    }

    fn is_in_bounds(&self, p: impl Point2d) -> bool {
        self.in_bounds(p)
    }

    fn set_visible(&mut self, p: impl Point2d) {
        if self.in_bounds(p) {
            self[p] |= VisibilityFlag::VISIBLE;
        }
    }
}

impl VisibilityMapUtility for VisibilityMap2d {
    fn toggle_opaque(&mut self, p: impl Point2d) {
        let i = self.get_idx_unchecked(p);
        self[i].toggle(VisibilityFlag::OPAQUE)
    }

    fn toggle_visible(&mut self, p: impl Point2d) {
        let i = self.get_idx_unchecked(p);
        self[i].toggle(VisibilityFlag::VISIBLE)
    }

    fn clear_all(&mut self) {
        self.iter_mut().for_each(|v| *v &= VisibilityFlag::NONE);
    }

    fn clear_opaque(&mut self) {
        self.iter_mut().for_each(|p| p.remove(VisibilityFlag::OPAQUE));
    }

    fn clear_visible(&mut self) {
        self.iter_mut().for_each(|p| p.remove(VisibilityFlag::VISIBLE));
    }
}
