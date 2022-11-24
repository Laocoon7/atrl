use crate::prelude::*;
pub trait GridShape {
    /// returns the number of points in the shape
    fn get_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: impl Point2d) -> bool;

    /// returns an iterator over all of the points
    fn get_points(&self) -> HashSet<IVec2>;
}

pub trait GridShapeWithBorder: GridShape {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: impl Point2d) -> bool;

    /// returns an iterator over all of the points
    fn get_border_points(&self) -> HashSet<IVec2>;
}
