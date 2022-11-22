// FoV implementation taken from:
// https://www.albertford.com/shadowcasting/

use super::super::shared::*;
use super::{quadrant::*, row::*};
use crate::prelude::*;

pub struct Shadowcast;

impl FovAlgorithm for Shadowcast {
    fn compute_fov(
        origin: IVec2,
        vision_type: u8,
        range: u32,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        receiver.set_visible(origin);

        CardinalDirection::all().enumerate().for_each(|(_index, direction)| {
            let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
            let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row);
        });
    }
}

impl Shadowcast {
    pub fn compute_direction(
        origin: IVec2,
        vision_type: u8,
        range: u32,
        provider: &impl FovProvider,
        receiver: &mut impl FovReceiver,
        direction: CardinalDirection,
    ) {
        receiver.set_visible(origin);

        let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
        let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
        Self::scan_recursive(range, &mut quadrant, &mut first_row);
    }

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
