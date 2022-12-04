use std::ops::Div;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRectangle {
    pub min: IVec2,
    pub max: IVec2,
}

impl Default for GridRectangle {
    fn default() -> Self { Self::new_with_size(IVec2::ZERO, IVec2::ONE) }
}

impl GridRectangle {
    #[inline]
    pub fn new(min: impl GridPoint, max: impl GridPoint) -> Self {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    #[inline]
    pub fn new_with_size(min: impl GridPoint, size: impl Size2d) -> Self {
        let min = min.as_ivec2();
        Self::new(min, min + size.as_ivec2())
    }
}

impl GridRectangle {
    #[inline]
    pub const fn width(&self) -> i32 { self.max.x - self.min.x }

    #[inline]
    pub const fn height(&self) -> i32 { self.max.y - self.min.y }

    #[inline]
    pub const fn min(&self) -> IVec2 { self.min }

    #[inline]
    pub const fn max(&self) -> IVec2 { self.max }

    #[inline]
    pub fn is_square(&self) -> bool {
        let diff = self.max - self.min;
        diff.x == diff.y
    }
}

impl GridRectangle {
    #[inline]
    fn center(&self) -> IVec2 { self.min.mid_point(self.max) }

    #[inline]
    fn left(&self) -> i32 { self.min.x.min(self.max.x) }

    #[inline]
    fn right(&self) -> i32 { self.min.x.max(self.max.x) }

    #[inline]
    fn top(&self) -> i32 { self.min.y.max(self.max.y) }

    #[inline]
    fn bottom(&self) -> i32 { self.min.y.min(self.max.y) }

    /// Create a circle around the center to the closest edge
    #[inline]
    pub fn as_smallest_circle(&self) -> Circle {
        let radius = self.width().div(2).min(self.height().div(2)) as u32;
        Circle::new(self.center(), radius)
    }

    /// Create a circle around the center to the farthest edge
    #[inline]
    pub fn as_biggest_circle(&self) -> Circle {
        let radius = self.width().div(2).max(self.height().div(2)) as u32;
        Circle::new(self.center(), radius)
    }

    #[inline]
    pub fn as_triangles(&self) -> (Triangle, Triangle) {
        let bottom_left = IVec2::new(self.left(), self.bottom());
        let top_left = IVec2::new(self.left(), self.top());
        let bottom_right = IVec2::new(self.right(), self.bottom());
        let top_right = IVec2::new(self.right(), self.top());
        (
            Triangle::new(bottom_left, top_right, top_left),
            Triangle::new(bottom_left, bottom_right, top_right),
        )
    }

    #[inline]
    pub fn as_polygon(&self) -> Polygon {
        let max = IVec2::new(self.right(), self.top());
        let bottom_left = IVec2::new(self.left(), self.bottom());
        Polygon::new(vec![self.min, max, self.max, bottom_left])
    }

    // #[inline]
    // pub fn as_ellipse(&self) -> Ellipse { Ellipse::from_points(self.points()) }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        self.min.x <= other.max.x &&
            self.max.x >= other.min.x &&
            self.min.y <= other.max.y &&
            self.max.y >= other.min.y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where F: FnMut(IVec2) {
        RectIter::new(self.min, self.max).for_each(f);
    }
}

impl Shape for GridRectangle {
    #[inline]
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }

    #[inline]
    fn contains(&self, point: impl GridPoint) -> bool {
        self.min.x <= point.x() && self.max.x > point.x() && self.min.y <= point.y() && self.max.y > point.y()
    }

    #[inline]
    fn get_positions(&self) -> HashSet<IVec2> {
        let mut result = HashSet::new();
        for y in self.min.y..self.max.y {
            for x in self.min.x..self.max.x {
                result.insert(IVec2::new(x, y));
            }
        }
        result
    }
}

impl IntoIterator for Rectangle {
    type IntoIter = RectIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { RectIter::new(self.min, self.max) }
}

impl ShapeIter for Rectangle {
    type Iterator = RectIter;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}
