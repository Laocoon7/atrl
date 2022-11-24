use crate::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: IVec2,
    radius: u32,
}

impl Circle {
    pub fn new(center: impl Point2d, radius: u32) -> Self {
        Self {
            center: center.as_ivec2(),
            radius,
        }
    }
}

impl Circle {
    /// Radius of circle
    ///
    /// Distance from center to edge
    #[inline]
    pub const fn radius(&self) -> u32 { self.radius }
}

impl Shape for Circle {
    /// must be [center, edge]
    fn from_points(points: Vec<impl Point2d>) -> Self
    where Self: Sized {
        debug_assert!(points.len() >= 2);
        let radius = DistanceAlg::Pythagoras.distance2d(points[0], points[1]).floor() as u32; // .round() ??
        Self::new(points[0], radius)
    }

    fn translate_by(&self, delta: impl Point2d) -> Self {
        Self::new(self.center + delta.as_ivec2(), self.radius)
    }

    fn move_to(&self, point: impl Point2d) -> Self { Self::new(point.as_ivec2(), self.radius) }

    fn contains(&self, point: impl Point2d) -> bool {
        let dist = DistanceAlg::Pythagoras.distance2d(self.center, point).floor() as u32; // .round() ??
        dist <= self.radius
    }

    /// Returns [center, edge_at_0_degrees]
    fn points(&self) -> Vec<IVec2> {
        vec![
            self.center,
            Point2d::as_ivec2(&IVec2::from_angle(self.center, self.radius as f32, 0.0)),
        ]
    }

    #[inline]
    fn center(&self) -> IVec2 { self.center }

    #[inline]
    fn left(&self) -> i32 { self.center.x - self.radius as i32 }

    #[inline]
    fn right(&self) -> i32 { self.center.x + self.radius as i32 }

    #[inline]
    fn top(&self) -> i32 { self.center.y - self.radius as i32 }

    #[inline]
    fn bottom(&self) -> i32 { self.center.y + self.radius as i32 }
}

impl Circle {
    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    /// Create line from center to edge at 0 degrees
    pub fn as_radius_line(&self) -> Line { Line::from_points(self.points()) }

    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.bottom()), (self.center.x, self.top()))
    }

    pub fn as_ellipse(&self) -> Ellipse { Ellipse::new(self.center, [self.radius * 2, self.radius * 2]) }
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
