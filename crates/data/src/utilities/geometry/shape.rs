use crate::prelude::*;
pub trait Shape: ShapeIter {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: Position) -> bool;

    /// returns an iterator over all of the points
    #[inline]
    fn get_positions(&self) -> HashSet<Position> { self.iter().collect::<HashSet<Position>>() }
}

pub trait ShapeWithBorder: Shape {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: Position) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<Position>;
}
