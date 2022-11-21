use crate::prelude::*;
use arrayvec::ArrayVec;
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
    type ExitIterator = arrayvec::IntoIter<IVec2, 8>;

    fn successors(&self, p: impl Point2d) -> Self::ExitIterator {
        let mut points = ArrayVec::new();

        for adj in p.adj_8() {
            println!("get_bit_at: {:?}", adj);
            if !self.get_bit_at(adj) {
                points.push(adj);
            }
        }

        points.into_iter()
    }

    fn distance(&self, a: impl Point2d, b: impl Point2d) -> OrderedFloat<f32> {
        a.taxi_dist(b).into()
    }
}
