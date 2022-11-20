use crate::prelude::*;
use arrayvec::{ArrayVec, IntoIter};
use ordered_float::OrderedFloat;

/// A pathmap represented as a 2d grid of [bool].
///
/// Note that a grid position is considered an obstacle if it is set to `true`.
///
/// # Example
/// ```rust
///
/// let mut map = PathMap2d::new([50,50]);
/// let mut pf = Pathfinder::new();
///
/// // Set position [5,4] of the path map to be a pathfinding obstacle.
/// map[[5,4]] = true;
///
/// let path = pf.astar(&map, [4,4], [10,10]).unwrap();
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
