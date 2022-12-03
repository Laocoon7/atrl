use super::{grid_shape::*, line::Line};
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: IVec2,
    radius: u32,
}

impl Circle {
    pub fn new<R: Into<u32>>(center: impl GridPoint, radius: R) -> Self {
        Self {
            radius: radius.into(),
            center: center.as_ivec2(),
        }
    }
}

impl Circle {
    #[inline]
    pub const fn center(&self) -> IVec2 { self.center }

    #[inline]
    pub const fn left(&self) -> i32 { self.center.x - self.radius as i32 }

    #[inline]
    pub const fn right(&self) -> i32 { self.center.x + self.radius as i32 }

    #[inline]
    pub const fn top(&self) -> i32 { self.center.y - self.radius as i32 }

    #[inline]
    pub const fn bottom(&self) -> i32 { self.center.y + self.radius as i32 }

    #[inline]
    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Create line from center to edge at 0 degrees
    // #[inline]
    // pub fn as_radius_line(&self) -> Line { Line::from_points(self.get_points()) }

    #[inline]
    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    #[inline]
    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.bottom()), (self.center.x, self.top()))
    }

    #[inline]
    pub fn as_ellipse(&self) -> Ellipse { Ellipse::new(self.center, [self.radius * 2, self.radius * 2]) }
}

impl GridShape for Circle {
    #[inline]
    // FIX: PERF
    fn get_count(&self) -> u32 { self.get_points().len() as u32 }

    #[inline]
    fn contains(&self, point: impl GridPoint) -> bool { self.get_points().contains(&point.as_ivec2()) }

    #[inline]
    fn get_points(&self) -> HashSet<IVec2> {
        let mut discovered = HashSet::new();
        let mut d = (5 - (self.radius as i32 * 4)) / 4;
        let mut x = 0;
        let mut y = self.radius as i32;

        loop {
            let line = Line::new(
                (self.center.x + x, self.center.y + y),
                (self.center.x - x, self.center.y + y),
            );

            for point in line.get_points() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x - x, self.center.y - y),
                (self.center.x + x, self.center.y - y),
            );

            for point in line.get_points() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x + y, self.center.y + x),
                (self.center.x - x, self.center.y + x),
            );

            for point in line.get_points() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x + y, self.center.y - x),
                (self.center.x - x, self.center.y - x),
            );

            for point in line.get_points() {
                discovered.insert(point);
            }

            if d < 0 {
                d += (2 * x) + 1;
            } else {
                d += (2 * (x - y)) + 1;
                y -= 1;
            }
            x += 1;

            if x > y {
                break;
            }
        }
        discovered
    }
}

impl IntoIterator for Circle {
    type IntoIter = BresenhamCircleIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { BresenhamCircleIter::new(self.center, self.radius as i32) }
}

impl ShapeIter for Circle {
    type Iterator = BresenhamCircleIter;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.into_iter() }
}
