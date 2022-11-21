use crate::prelude::*;
use arrayvec::{ArrayVec, IntoIter};
use ordered_float::OrderedFloat;

/// A pathmap represented as a 2d grid of [bool].
///
/// Note that a grid position is considered an obstacle if it is set to `true`.
///
/// # Example
/// ```rust
/// use crate::prelude::*;
///
/// let mut map = PathMap2d::new([50,50]);
///
/// // Set position [5,4] of the path map to be a pathfinding obstacle.
/// map[[5,4]] = true;
///
/// let path = pathfinder::astar(&map, [4,4], [10,10]);
/// if let Some(path) = path {...}
/// ```
pub type PathMap2d = Grid<u8>;

impl PathMap for PathMap2d {
    type ExitIterator = IntoIter<(IVec2, OrderedFloat<f32>), 8>;

    fn successors(&self, p: &impl Point2d) -> Self::ExitIterator {
        let mut points = ArrayVec::new();

        for adj in p.adj_8() {
            if !self.in_bounds(adj) {
                continue;
            }

            if !self.get_bit_at(adj) {
                points.push((adj, self.cost(adj)));
            }
        }

        points.into_iter()
    }

    fn cost(&self, _: impl Point2d) -> OrderedFloat<f32> {
        OrderedFloat(1.0)
    }

    fn distance(&self, a: impl Point2d, b: impl Point2d) -> OrderedFloat<f32> {
        a.taxi_dist(b).into()
    }
}

pub trait BitPacker {
    fn new_packer(size: impl Size2d) -> Grid<u8>;
    fn new_packer_with(size: impl Size2d, value: u8) -> Grid<u8>;
    fn new_packer_fn<F>(size: impl Size2d, f: F) -> Grid<u8>
    where
        F: FnMut(IVec2) -> u8;

    fn get_bit_at(&self, p: impl Point2d) -> bool;
    fn set_bit_at(&mut self, p: impl Point2d);
    fn clear_bit_at(&mut self, p: impl Point2d);
}

impl BitPacker for Grid<u8> {
    fn new_packer(size: impl Size2d) -> Grid<u8> {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Self::new_default([width, size.height()])
    }

    fn new_packer_with(size: impl Size2d, value: u8) -> Grid<u8> {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Self::new_clone([width, size.height()], value)
    }

    fn new_packer_fn<F>(size: impl Size2d, f: F) -> Grid<u8>
    where
        F: FnMut(IVec2) -> u8,
    {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Self::new_fn([width, size.height()], f)
    }

    fn get_bit_at(&self, p: impl Point2d) -> bool {
        if p.x() < 0 {
            return false;
        }
        self.get((p.x() / 8, p.y())).map_or(false, |byte| (*byte >> (p.x() % 8)) & 0b0000_0001 == 1)
    }

    fn set_bit_at(&mut self, p: impl Point2d) {
        if p.x() < 0 {
            return;
        }
        if let Some(byte) = self.get((p.x() / 8, p.y())) {
            self.set((p.x() / 8, p.y()), *byte | (1 << ((p.x().max(0)) % 8)) as u8);
        }
    }

    fn clear_bit_at(&mut self, p: impl Point2d) {
        if p.x() < 0 {
            return;
        }
        if let Some(byte) = self.get((p.x() / 8, p.y())) {
            self.set((p.x() / 8, p.y()), *byte & !(1 << ((p.x().max(0)) % 8)));
        }
    }
}
