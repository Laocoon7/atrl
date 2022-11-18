use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Ellipse {
    center: IVec2,
    size: UVec2,
}

impl Ellipse {
    pub fn new(center: impl Point2d, size: impl Size2d) -> Self {
        Self { center: center.as_ivec2(), size: size.as_uvec2() }
    }
}

impl Ellipse {
    #[inline]
    pub fn width(&self) -> u32 {
        self.size.x
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.size.y
    }
}

impl Shape for Ellipse {
    /// must be [top_left, bottom_right]
    fn from_points(points: Vec<impl Point2d>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        let width = points[1].x() - points[0].x();
        let height = points[1].y() - points[0].x();
        let center = points[0].mid_point(points[1]);
        Ellipse::new(center, [width.max(0), height.max(0)])
    }

    fn translate_by(&self, delta: impl Point2d) -> Self {
        Ellipse::new(self.center + delta.as_ivec2(), self.size)
    }

    fn move_to(&self, point: impl Point2d) -> Self {
        Ellipse::new(point, self.size)
    }

    fn contains(&self, point: impl Point2d) -> bool {
        ((point.x() - self.center.x) ^ 2) / ((self.size.x as i32) ^ 2)
            + ((point.y() - self.center.y) ^ 2) / ((self.size.y as i32) ^ 2)
            <= 1
    }

    /// Returns [top_left, bottom_right]
    fn points(&self) -> Vec<IVec2> {
        vec![IVec2::new(self.left(), self.top()), IVec2::new(self.right(), self.bottom())]
    }

    #[inline]
    fn center(&self) -> IVec2 {
        self.center
    }

    #[inline]
    fn left(&self) -> i32 {
        self.center.x - (self.size.x as i32) / 2
    }

    #[inline]
    fn right(&self) -> i32 {
        self.center.x + (self.size.x as i32) / 2
    }

    #[inline]
    fn top(&self) -> i32 {
        self.center.y - (self.size.y as i32) / 2
    }

    #[inline]
    fn bottom(&self) -> i32 {
        self.center.y + (self.size.y as i32) / 2
    }
}

impl Ellipse {
    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new((self.left(), self.top()), (self.right(), self.bottom()))
    }

    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.top()), (self.center.x, self.bottom()))
    }

    /// Create line from center to right edge at 0 degrees
    pub fn as_radius_line(&self) -> Line {
        Line::new((self.center.x, self.center.y), (self.right(), self.center.y))
    }

    pub fn as_circle(&self) -> Option<Circle> {
        if self.size.x == self.size.y {
            Some(Circle::new(self.center, self.size.x / 2))
        } else {
            None
        }
    }
}
