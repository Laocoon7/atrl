// FoV implementation taken from:
// https://www.albertford.com/shadowcasting/

use super::{quadrant::*, row::*, slope::*};
use crate::prelude::*;

pub struct AFFov;

impl Fov for AFFov {
    fn compute_fov<Range: Into<u32>>(
        origin: impl Point2d,
        vision: VisionType,
        range: Range,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        let origin = origin.as_ivec2();
        let range = range.into();

        receiver.set_visible(origin);

        CardinalDirection::all().enumerate().for_each(|(_index, direction)| {
            let mut quadrant =
                Quadrant::new(direction, origin, vision, Box::new(provider), Box::new(receiver));
            let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row);
        });
    }
}

impl AFFov {
    fn scan_recursive(range: u32, quadrant: &mut Quadrant, row: &mut Row) {
        let range_sqr = range.pow(2) as f32;

        let mut prev_tile = None;
        for tile in row.tiles() {
            if DistanceAlg::PythagorasSquared.distance2d(IVec2::ZERO, tile) > range_sqr {
                continue;
            }

            // Should we reveal the tile?
            if quadrant.is_opaque(tile) | row.is_symmetric(tile) {
                quadrant.set_visible(tile);
            }

            // handle the current row based on vision angles around the previous tile
            if let Some(prev_tile) = prev_tile {
                // did we *just* hit floor after traveling through walls?
                if quadrant.is_opaque(prev_tile) & quadrant.is_clear(tile) {
                    row.calc_starting_slope(tile)
                }

                // did we *just* hit a wall after traveling through floors?
                if quadrant.is_clear(prev_tile) & quadrant.is_opaque(tile) {
                    let mut next_row = row.next();
                    next_row.calc_ending_slope(tile);
                    Self::scan_recursive(range, quadrant, &mut next_row);
                }
            }
            // setup for next tile
            prev_tile = Some(tile);
        }

        // if our last tile was floor, we can see down another row
        if let Some(prev_tile) = prev_tile {
            if quadrant.is_clear(prev_tile) {
                Self::scan_recursive(range, quadrant, &mut row.next());
            }
        }
    }
}
