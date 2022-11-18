use crate::prelude::*;
use std::ops::Div;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Rectangle {
    min: IVec2,
    max: IVec2,
}

impl Rectangle {
    #[inline]
    pub fn new(min: impl Point2d, max: impl Point2d) -> Self {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        Self { min: min.min(max), max: min.max(max) }
    }

    #[inline]
    pub fn new_with_size(min: impl Point2d, size: impl Size2d) -> Self {
        let min = min.as_ivec2();
        Self::new(min, min + size.as_ivec2())
    }
}

impl Rectangle {
    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn min(&self) -> IVec2 {
        self.min
    }

    #[inline]
    pub fn max(&self) -> IVec2 {
        self.max
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        let diff = self.max - self.min;
        diff.x == diff.y
    }
}

impl Shape for Rectangle {
    fn from_points(points: Vec<impl Point2d>) -> Self
    where
        Self: Sized,
    {
        Rectangle::new(points[0], points[1])
    }

    fn contains(&self, point: impl Point2d) -> bool {
        self.min.x <= point.x()
            && self.max.x > point.x()
            && self.min.y <= point.y()
            && self.max.y > point.y()
    }

    fn points(&self) -> Vec<IVec2> {
        vec![self.min, self.max]
    }

    fn rotate_around(&self, point: impl Point2d, degrees: f32) -> Self
    where
        Self: Sized,
    {
        let points = rotate_points(point, &self.points(), degrees);
        Self::from_points(points)
    }

    fn center(&self) -> IVec2 {
        self.min.mid_point(self.max)
    }

    fn left(&self) -> i32 {
        self.min.x.min(self.max.x)
    }

    fn right(&self) -> i32 {
        self.min.x.max(self.max.x)
    }

    fn top(&self) -> i32 {
        self.min.y.max(self.max.y)
    }

    fn bottom(&self) -> i32 {
        self.min.y.min(self.max.y)
    }
}

impl Rectangle {
    /// Create a circle around the center to the closest edge
    pub fn as_smallest_circle(&self) -> Circle {
        let radius = self.width().div(2).min(self.height().div(2)) as u32;
        Circle::new(self.center(), radius)
    }

    /// Create a circle around the center to the farthest edge
    pub fn as_biggest_circle(&self) -> Circle {
        let radius = self.width().div(2).max(self.height().div(2)) as u32;
        Circle::new(self.center(), radius)
    }

    pub fn as_triangles(&self) -> (Triangle, Triangle) {
        let max = IVec2::new(self.right(), self.top());
        let min = IVec2::new(self.left(), self.bottom());
        (Triangle::new(self.min(), max, min), Triangle::new(self.max(), max, min))
    }

    pub fn as_polygon(&self) -> Polygon {
        let max = IVec2::new(self.right(), self.top());
        let bottom_left = IVec2::new(self.left(), self.bottom());
        Polygon::new(vec![self.min, max, self.max, bottom_left])
    }

    pub fn as_ellipse(&self) -> Ellipse {
        Ellipse::from_points(self.points())
    }
}

impl Rectangle {
    /// Check if this rectangle intersects another rectangle.
    #[inline]
    #[must_use]
    pub fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()

        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Gets a set of all tiles in the rectangle
    #[must_use]
    #[inline]
    pub fn point_set(&self) -> HashSet<IVec2> {
        let mut result = HashSet::new();
        for y in self.min.y..self.max.y {
            for x in self.min.x..self.max.x {
                result.insert(IVec2::new(x, y));
            }
        }
        result
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(IVec2),
    {
        RectPointIter::new(self.min, self.max).for_each(f);
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new_with_size(IVec2::ZERO, IVec2::ONE)
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct RectPointIter {
    curr: IVec2,
    size: IVec2,

    /// The minimum corner point of the rect.
    pub min: IVec2,
    /// The maximum corner point of the rect.
    pub max: IVec2,
}

impl RectPointIter {
    pub fn new(min: impl Size2d, max: impl Size2d) -> Self {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        let size = max - min;
        Self { min, max, size, curr: IVec2::ZERO }
    }
}

impl Iterator for RectPointIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.cmpge(self.max).any() {
            return None;
        }

        let p = self.curr;
        self.curr.x += 1;
        if self.curr.x == self.size.x {
            self.curr.x = 0;
            self.curr.y += 1;
        }
        Some(self.min + p)
    }
}

impl IntoIterator for Rectangle {
    type Item = IVec2;
    type IntoIter = RectPointIter;

    fn into_iter(self) -> Self::IntoIter {
        RectPointIter::new(self.min, self.max)
    }
}

impl From<Rectangle> for RectPointIter {
    fn from(rect: Rectangle) -> Self {
        rect.into_iter()
    }
}
