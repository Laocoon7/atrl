use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Rectangle {
    pub min: IVec2,
    pub max: IVec2,
}

impl Rectangle {
    /// Create a new rectangle from top corner and size.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    #[inline]
    pub fn new(top_corner: impl Size2d, size: impl Size2d) -> Self {
        let top_corner = top_corner.as_ivec2();
        Self::from_corners(top_corner, top_corner + size.as_ivec2())
    }

    /// Create a new rectangle from two corner points.
    ///
    /// The two points do not need to be the minimum and/or maximum corners.
    /// They only need to be two opposite corners.
    #[inline]
    pub fn from_corners(top_corner: impl Size2d, bottom_corner: impl Size2d) -> Self {
        let top_corner = top_corner.as_ivec2();
        let bottom_corner = bottom_corner.as_ivec2();
        Self { min: top_corner, max: bottom_corner }
    }

    /// Rectangle width (max.x - min.x).
    #[inline]
    #[must_use]
    pub fn width(&self) -> i32 { self.max.x - self.min.x }

    /// Rectangle height (max.y - min.y).
    #[inline]
    #[must_use]
    pub fn height(&self) -> i32 { self.max.y - self.min.y }

    /// Rectangle size.
    #[inline]
    #[must_use]
    pub fn size(&self) -> IVec2 { self.max - self.min }

    /// The center point of the rectangle.
    #[inline]
    #[must_use]
    pub fn center(&self) -> IVec2 { (self.min + self.max) / 2 }

    /// Check if a point lies within this rectangle, inclusive of its edges.
    #[inline]
    pub fn contains(&self, point: impl Point2d) -> bool {
        let point = point.as_ivec2();
        (point.cmpge(self.min) & point.cmple(self.max)).all()
    }

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
    fn default() -> Self { Self::new(IVec2::ZERO, IVec2::ONE) }
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

    fn into_iter(self) -> Self::IntoIter { RectPointIter::new(self.min, self.max) }
}

impl From<Rectangle> for RectPointIter {
    fn from(rect: Rectangle) -> Self { rect.into_iter() }
}

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Neg
impl Neg for Rectangle {
    type Output = Rectangle;
    fn neg(self) -> Self::Output { Rectangle { min: -self.min, max: -self.max } }
}

// Add + Assign
impl Add<Rectangle> for Rectangle {
    type Output = Self;
    fn add(self, Rectangle { min, max }: Rectangle) -> Rectangle {
        Self { min: self.min + min, max: self.max + max }
    }
}

impl<P: Point2d> Add<P> for Rectangle {
    type Output = Self;
    fn add(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rectangle { min: self.min + rhs.x, max: self.max + rhs.x }
    }
}

impl<T> AddAssign<T> for Rectangle
where
    Rectangle: Add<T, Output = Rectangle>,
{
    fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }
}

// Sub + Assign
impl Sub<Rectangle> for Rectangle {
    type Output = Self;
    fn sub(self, rhs: Rectangle) -> Self::Output { self + -rhs }
}

impl<P: Point2d> Sub<P> for Rectangle {
    type Output = Self;
    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rectangle { min: self.min - rhs.x, max: self.max - rhs.x }
    }
}

impl<T> SubAssign<T> for Rectangle
where
    Rectangle: Sub<T, Output = Rectangle>,
{
    fn sub_assign(&mut self, rhs: T) { *self = *self - rhs; }
}

// Mul + MulAssign
impl Mul<Rectangle> for Rectangle {
    type Output = Rectangle;
    fn mul(self, rhs: Rectangle) -> Self::Output {
        Rectangle { min: self.min * rhs.min, max: self.max * rhs.max }
    }
}

impl<P: Point2d> Mul<P> for Rectangle {
    type Output = Rectangle;
    fn mul(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rectangle { min: self.min * rhs, max: self.max * rhs }
    }
}

impl<T> MulAssign<T> for Rectangle
where
    Rectangle: Mul<T, Output = Rectangle>,
{
    fn mul_assign(&mut self, rhs: T) { *self = *self * rhs; }
}

// Div + DivAssign

impl Div<Rectangle> for Rectangle {
    type Output = Rectangle;
    fn div(self, rhs: Rectangle) -> Self::Output {
        Rectangle { min: self.min / rhs.min, max: self.max / rhs.max }
    }
}

impl<P: Point2d> Div<P> for Rectangle {
    type Output = Rectangle;
    fn div(self, rhs: P) -> Self::Output {
        let rhs = rhs.as_ivec2();
        Rectangle { min: self.min / rhs, max: self.max / rhs }
    }
}

impl<T> DivAssign<T> for Rectangle
where
    Rectangle: Div<T, Output = Rectangle>,
{
    fn div_assign(&mut self, rhs: T) { *self = *self / rhs; }
}
