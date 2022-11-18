//! [![License: BlueOak-1.0.0](https://img.shields.io/badge/License-BlueOak--1.0.0-blue)](https://opensource.org/licenses/MIT)
//! An implementation of [Adam Millazo's FOV algorithm](http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html#mine)
//!
//! //! To use it you must implement the [VisibilityMap] trait on your map type, or use
//! the build in [VisibilityMap2d]. Then you can call [fov::compute] with your map
//! which will populate visible tiles based on the map's opaque tiles.
//!
//! # Example
//! ```rust
//! use adam_fov_rs::*;
//!
//! // Create a 50x50 visibility map
//! let mut map = VisibilityMap2d::default([50,50]);
//!
//! // Set the tile at (15,15) to opaque
//! map[[15,15]].opaque = true;
//!
//! // Compute our visible tiles and add them to the map
//! fov::compute([15,14], 5, &mut map);
//!
//! // The space directly above our opaque tile is not visible
//! assert!(map[[15,16]].visible == false);
//! ```

use crate::prelude::*;

pub type VisibilityMap2d = Grid<VisibilityPoint>;

/// A point in the visibility map.
///
/// This is used to store the visibility state of a point in the map.
#[derive(Default, Debug, Clone, Copy, Reflect, FromReflect)]
pub struct VisibilityPoint {
    pub opaque: bool,
    pub visible: bool,
}

/// Compute the fov in a map from the given position.
pub fn compute<T: VisibilityMap>(origin: impl Point2d, range: i32, map: &mut T) {
    let origin = origin.as_ivec2();
    map.set_visible(origin);

    GridDirection::all().enumerate().for_each(|(octant, _)| {
        compute_octant(octant, origin, range, 1, Slope { x: 1, y: 1 }, Slope { x: 1, y: 0 }, map)
    });
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_fov() {
        let mut map = VisibilityMap2d::new_default([30, 30]);
        map[(0, 1)].opaque = true;
        map[(1, 0)].opaque = true;
        fov::compute((0, 0), 5, &mut map);

        assert!(map[(0, 0)].visible);

        assert!(map[(0, 1)].visible);
        assert!(!map[(0, 2)].visible);

        assert!(map[(1, 0)].visible);
        assert!(!map[(2, 0)].visible);
    }
}
