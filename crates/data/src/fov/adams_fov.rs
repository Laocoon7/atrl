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

/// Compute the fov in a map from the given position.
pub fn compute<P: VisibilityProvider, M: VisibilityMap>(
    origin: impl Point2d,
    range: u32,
    vision_component: &Vision,
    visibility_provider: &P,
    visibility_map: &mut M,
) {
    let origin = origin.as_ivec2();
    visibility_map.set_visible(origin);

    let range = range as i32;
    GridDirection::all().enumerate().for_each(|(octant, _)| {
        compute_octant(
            1,
            &mut VisibilityData {
                visibility_provider,
                visibility_map,
                range,
                octant,
                origin,
                vision_component,
                top: &mut Slope { x: 1, y: 1 },
                bottom: &mut Slope { x: 1, y: 0 },
            },
        )
    });
}
