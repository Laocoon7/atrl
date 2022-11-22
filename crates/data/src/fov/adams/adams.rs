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

pub use super::super::shared::*;
use super::octant::*;
use crate::prelude::*;

pub(crate) struct AdamsFov;

impl FovAlgorithm for AdamsFov {
    fn compute_fov(
        origin: IVec2,
        vision_type: u8,
        range: u32,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        receiver.set_visible(origin);

        GridDirection::all().enumerate().for_each(|(_index, direction)| {
            let mut octant =
                Octant::new(direction, origin, vision_type, Box::new(provider), Box::new(receiver));
            let mut top = Slope::new(1, 1);
            let mut bottom = Slope::new(0, 1);
            Self::compute_octant(1, range as i32, &mut top, &mut bottom, &mut octant);
        })
    }
}

impl AdamsFov {
    pub fn compute_direction(
        origin: IVec2,
        vision_type: u8,
        range: u32,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
        direction: GridDirection,
    ) {
        receiver.set_visible(origin);

        let mut octant =
            Octant::new(direction, origin, vision_type, Box::new(provider), Box::new(receiver));
        let mut top = Slope::new(1, 1);
        let mut bottom = Slope::new(0, 1);
        Self::compute_octant(1, range as i32, &mut top, &mut bottom, &mut octant);
    }

    fn compute_octant(
        start: i32,
        range: i32,
        top: &mut Slope,
        bottom: &mut Slope,
        octant: &mut Octant,
    ) {
        for row in start..=range {
            let column_limits = Self::get_column_limits(row, *top, *bottom, octant);
            if Self::scan_column(range, row, column_limits, top, bottom, octant) {
                break;
            }
        }
    }

    fn get_column_limits(row: i32, top: Slope, bottom: Slope, octant: &mut Octant) -> (i32, i32) {
        let ret_top = if top.x == 1 {
            1
        } else {
            let mut y = ((row * 2 - 1) * top.y + top.x) / (top.x * 2);

            if octant.is_opaque(IVec2::new(row, y)) {
                if top.greater_or_equal(y * 2 + 1, row * 2)
                    && octant.is_clear(IVec2::new(row, y + 1))
                {
                    y += 1;
                }
            } else {
                let mut ax = row * 2;

                if octant.is_opaque(IVec2::new(row + 1, y + 1)) {
                    ax += 1;
                }

                if top.greater(y * 2 + 1, ax) {
                    y += 1
                }
            }

            y
        };

        let ret_bottom = if bottom.y == 0 {
            0
        } else {
            let mut y = ((row * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2);

            if bottom.greater_or_equal(y * 2 + 1, row * 2)
                && octant.is_opaque(IVec2::new(row, y))
                && octant.is_clear(IVec2::new(row, y + 1))
            {
                y += 1;
            }

            y
        };

        (ret_top, ret_bottom)
    }

    fn scan_column(
        range: i32,
        row: i32,
        column_limits: (i32, i32),
        top: &mut Slope,
        bottom: &mut Slope,
        octant: &mut Octant,
    ) -> bool {
        let mut was_opaque = None;
        let range_sqr = range.pow(2) as f32;

        for column in (column_limits.1..=column_limits.0).rev() {
            let tile = IVec2::new(row, column);

            if DistanceAlg::PythagorasSquared.distance2d(IVec2::ZERO, tile) > range_sqr {
                continue;
            }

            let is_opaque = octant.is_opaque(tile);
            let is_visible = is_opaque
                || ((column != column_limits.1 || top.greater_or_equal(column, row))
                    && (column != column_limits.0 || bottom.less_or_equal(column, row)));

            if is_visible {
                octant.set_visible(tile);
            }

            if row != range {
                if is_opaque {
                    match was_opaque {
                        Some(false) => {
                            let mut x = row * 2;
                            let y = column * 2 + 1;
                            if octant.is_opaque(IVec2::new(row, column + 1)) {
                                x -= 1;
                            }
                            if top.greater(y, x) {
                                if column == column_limits.1 {
                                    *bottom = Slope::new(y, x);
                                    break;
                                } else {
                                    Self::compute_octant(row + 1, range, top, bottom, octant);
                                }
                            } else if column == column_limits.1 {
                                return true;
                            }
                        }
                        _ => (),
                    }
                    was_opaque = Some(true);
                } else {
                    match was_opaque {
                        Some(true) => {
                            let mut x = row * 2;
                            let y = column * 2 + 1;
                            if octant.is_opaque(IVec2::new(row + 1, column + 1)) {
                                x += 1;
                            }
                            if bottom.greater_or_equal(y, x) {
                                return true;
                            }
                            *top = Slope::new(y, x);
                        }
                        _ => (),
                    }
                    was_opaque = Some(false);
                }
            }
        }

        was_opaque != Some(false)
    }
}
