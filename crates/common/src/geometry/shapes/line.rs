use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq,)]
pub enum LineType {
    Point,
    Angled,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq,)]
pub struct Line {
    len: u32,
    angle: f32,
    line_type: LineType,
    pub(crate) end: IVec2,
    pub(crate) start: IVec2,
}

impl Line {
    pub fn new(start: impl Point2d, end: impl Point2d,) -> Self {
        let start = start.as_ivec2();
        let end = end.as_ivec2();
        let line_type = if start == end {
            LineType::Point
        } else if start.x() == end.x {
            LineType::Horizontal
        } else if start.y() == end.y {
            LineType::Vertical
        } else {
            LineType::Angled
        };

        let len = DistanceAlg::Pythagoras.distance2d(start, end,).floor() as u32; // round() ??
        let angle = start.angle_to(end,);
        Self { start, end, len, line_type, angle, }
    }
}

impl Line {
    #[allow(clippy::len_without_is_empty)] //use start()==end() to check that
    #[inline]
    pub const fn len(&self,) -> u32 { self.len }

    #[inline]
    pub const fn angle(&self,) -> f32 { self.angle }

    #[inline]
    pub const fn start(&self,) -> IVec2 { self.start }

    #[inline]
    pub const fn end(&self,) -> IVec2 { self.end }

    #[inline]
    pub const fn line_type(&self,) -> LineType { self.line_type }
}

impl Shape for Line {
    fn from_points(points: Vec<impl Point2d,>,) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        Self::new(points[0], points[1],)
    }

    fn contains(&self, point: impl Point2d,) -> bool {
        let point = point.as_ivec2();
        match self.line_type {
            LineType::Point => self.start == point,
            LineType::Horizontal => {
                self.start.y() == point.y && self.start.x() <= point.x && point.x <= self.end.x
            }
            LineType::Vertical => {
                self.start.x() == point.x && self.start.y() <= point.y && point.y <= self.end.y
            }
            LineType::Angled => {
                (DistanceAlg::Pythagoras.distance2d(self.start, point,) +
                    DistanceAlg::Pythagoras.distance2d(self.end, point,))
                .floor() as u32 ==
                    self.len
            } // TODO: CHECK THIS
        }
    }

    fn points(&self,) -> Vec<IVec2,> { vec![self.start, self.end] }

    fn center(&self,) -> IVec2 { self.start.mid_point(self.end,) }

    #[inline]
    fn left(&self,) -> i32 { self.start.x() }

    #[inline]
    fn right(&self,) -> i32 { self.end.x }

    #[inline]
    fn top(&self,) -> i32 { self.start.y() }

    #[inline]
    fn bottom(&self,) -> i32 { self.end.y }

    #[inline]
    fn iter(&self,) -> ShapeIterator
    where
        Self: std::fmt::Debug,
    {
        ShapeIterator::Line(self.into_iter(),)
    }

    #[inline]
    fn iter_exlusive(&self,) -> ShapeIteratorExclusive
    where
        Self: std::fmt::Debug,
    {
        ShapeIteratorExclusive::Line(self.into_iter_exlusive(),)
    }
}

impl Line {
    #[inline]
    fn into_iter_exlusive(self,) -> BresenhamLineInclusiveIter {
        BresenhamLineInclusiveIter::new(self.start, self.end,)
    }

    pub fn as_rect(&self,) -> Rectangle { Rectangle::new(self.start, self.end,) }

    pub fn as_circle(&self,) -> Circle { Circle::new(self.start, self.len,) }
}

impl IntoIterator for Line {
    type Item = IVec2;
    type IntoIter = BresenhamLineIter;

    fn into_iter(self,) -> Self::IntoIter { BresenhamLineIter::new(self.start, self.end,) }
}
