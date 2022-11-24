use crate::prelude::*;
pub trait Shape {
    /// create this shape from a list of points
    fn from_points(points: Vec<impl Point2d>) -> Self
    where Self: Sized;

    /// returns true if the shape contains point
    fn contains(&self, point: impl Point2d) -> bool;

    /// points(corners) the shape is made of
    fn points(&self) -> Vec<IVec2>;

    /// center of shape
    fn center(&self) -> IVec2;

    /// change every point by +`delta`
    fn translate_by(&self, delta: impl Point2d) -> Self
    where Self: Sized {
        let delta = delta.as_ivec2();
        let points = self.points().iter().map(|p| *p + delta).collect();
        Self::from_points(points)
    }

    /// moves the shapes first point to `point`
    /// (and changes every other point to match their original distance and angle)
    ///
    /// As this moves self.point[0] the result might be unexpected if the shape was created
    /// right to left and/or bottom to top
    fn move_to(&self, point: impl Point2d) -> Self
    where Self: Sized {
        let point = point.as_ivec2();
        let points = self.points().iter().map(|p| *p - point).collect();
        Self::from_points(points)
    }

    fn rotate(&self, degrees: f32) -> Self
    where Self: Sized {
        self.rotate_around(self.center(), degrees)
    }

    fn rotate_around(&self, point: impl Point2d, degrees: f32) -> Self
    where Self: Sized {
        let points = rotate_points(point, &self.points(), degrees);
        Self::from_points(points)
    }

    /// x of the left most point
    fn left(&self) -> i32 { self.points().iter().map(|p| p.x).min().unwrap() }

    /// x of the right most point
    fn right(&self) -> i32 { self.points().iter().map(|p| p.x).max().unwrap() }

    /// y of the top most point
    fn top(&self) -> i32 { self.points().iter().map(|p| p.y).max().unwrap() }

    /// y of the bottom most point
    fn bottom(&self) -> i32 { self.points().iter().map(|p| p.y).min().unwrap() }

    /// scale the shape by factor (around the center, so the change will be uniform)
    fn scale(&self, factor: f32) -> Self
    where Self: Sized {
        self.scale_around(self.center(), factor)
    }

    /// scale the shape by factor around point
    fn scale_around(&self, point: impl Point2d, factor: f32) -> Self
    where Self: Sized {
        let points = scale_points(point, &self.points(), factor);
        Self::from_points(points)
    }
}
