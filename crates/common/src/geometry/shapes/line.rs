use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineType {
    Point,
    Angled,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    len: u32,
    angle: f32,
    end: IVec2,
    start: IVec2,
    line_type: LineType,
}

impl Line {
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
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

        let len = DistanceAlg::Pythagoras.distance2d(start, end).floor() as u32; // round() ??
        let angle = start.angle_to(end);
        Self { start, end, len, line_type, angle }
    }
}

impl Line {
    #[allow(clippy::len_without_is_empty)] //use start()==end() to check that
    #[inline]
    pub const fn len(&self) -> u32 {
        self.len
    }

    #[inline]
    pub const fn angle(&self) -> f32 {
        self.angle
    }

    #[inline]
    pub const fn start(&self) -> IVec2 {
        self.start
    }

    #[inline]
    pub const fn end(&self) -> IVec2 {
        self.end
    }

    #[inline]
    pub const fn line_type(&self) -> LineType {
        self.line_type
    }
}

impl Shape for Line {
    fn from_points(points: Vec<impl Point2d>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        Self::new(points[0], points[1])
    }

    fn contains(&self, point: impl Point2d) -> bool {
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
                (DistanceAlg::Pythagoras.distance2d(self.start, point)
                    + DistanceAlg::Pythagoras.distance2d(self.end, point))
                .floor() as u32
                    == self.len
            } // TODO: CHECK THIS
        }
    }

    fn points(&self) -> Vec<IVec2> {
        vec![self.start, self.end]
    }

    fn center(&self) -> IVec2 {
        self.start.mid_point(self.end)
    }

    #[inline]
    fn left(&self) -> i32 {
        self.start.x()
    }

    #[inline]
    fn right(&self) -> i32 {
        self.end.x
    }

    #[inline]
    fn top(&self) -> i32 {
        self.start.y()
    }

    #[inline]
    fn bottom(&self) -> i32 {
        self.end.y
    }

    fn iter(&self) -> ShapeIterator
    where
        Self: std::fmt::Debug,
    {
        ShapeIterator::Line(self.into_iter())
    }
}

impl Line {
    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new(self.start, self.end)
    }

    pub fn as_circle(&self) -> Circle {
        Circle::new(self.start, self.len)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// First Line Algo
//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct GridLineIter {
    dist: f32,
    step: f32,
    end: IVec2,
    start: IVec2,
}

impl GridLineIter {
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
        let start = start.as_ivec2();
        let end = end.as_ivec2();
        Self { start, end, step: 0.0, dist: DistanceAlg::Diagonal.distance2d(start, end) }
    }
}

impl Iterator for GridLineIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > self.dist {
            return None;
        }

        let t = self.step as f32 / self.dist as f32;
        self.step += 1.0;

        Some(self.start.lerp(self.end, t))
    }
}

impl IntoIterator for Line {
    type Item = IVec2;
    type IntoIter = GridLineIter;

    fn into_iter(self) -> Self::IntoIter {
        GridLineIter::new(self.start, self.end)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

/// Line-drawing iterator
pub struct Bresenham {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x1: i32,
    diff: i32,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    #[inline]
    fn from_points(start: impl Point2d, end: impl Point2d) -> Self {
        let mut dx = end.x() - start.x();
        let mut dy = end.y() - start.y();

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }

        if dx < dy {
            octant += 1;
        }

        Self(octant)
    }

    #[inline]
    fn to_octant(&self, p: impl Point2d) -> IVec2 {
        match self.0 {
            0 => IVec2::new(p.x(), p.y()),
            1 => IVec2::new(p.y(), p.x()),
            2 => IVec2::new(p.y(), -p.x()),
            3 => IVec2::new(-p.x(), p.y()),
            4 => IVec2::new(-p.x(), -p.y()),
            5 => IVec2::new(-p.y(), -p.x()),
            6 => IVec2::new(-p.y(), p.x()),
            7 => IVec2::new(p.x(), -p.y()),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn from_point(&self, p: impl Point2d) -> IVec2 {
        match self.0 {
            0 => IVec2::new(p.x(), p.y()),
            1 => IVec2::new(p.y(), p.x()),
            2 => IVec2::new(-p.y(), p.x()),
            3 => IVec2::new(-p.x(), p.y()),
            4 => IVec2::new(-p.x(), -p.y()),
            5 => IVec2::new(-p.y(), -p.x()),
            6 => IVec2::new(p.y(), -p.x()),
            7 => IVec2::new(p.x(), -p.y()),
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant(start);
        let end = octant.to_octant(end);

        let dx = end.x() - start.x();
        let dy = end.y() - start.y();

        Self { x: start.x(), y: start.y(), dx, dy, x1: end.x(), diff: dy - dx, octant }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> IVec2 {
        let p = IVec2::new(self.x, self.y);

        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.dx;
        }

        self.diff += self.dy;

        // loop inc
        self.x += 1;

        self.octant.from_point(p)
    }
}

impl Iterator for Bresenham {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            None
        } else {
            Some(self.advance())
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham inclusive Algo
//////////////////////////////////////////////////////////////////////////////////////////

/// New type over `Bresenham` which include the `end` points when iterated over.
pub struct BresenhamInclusive(Bresenham);
impl BresenhamInclusive {
    /// Creates a new iterator. Yields points `start..=end`.
    #[inline]
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
        Self(Bresenham::new(start, end))
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> impl Point2d {
        self.0.advance()
    }
}
impl Iterator for BresenhamInclusive {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.x > self.0.x1 {
            None
        } else {
            Some(self.0.advance())
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod line {
        use crate::prelude::*;

        #[test]
        fn line_vertical() {
            let mut canvas = Canvas::new([11, 11]);
            let line = Line::new((0, 0), (0, 10));
            for p in line.iter() {
                canvas.put(p, '*');
            }
            canvas.print();
        }

        #[test]
        fn line_horizontal() {
            let mut canvas = Canvas::new([11, 11]);
            let line = Line::new((0, 0), (10, 0));
            for p in line.iter() {
                canvas.put(p, '*');
            }
            canvas.print();
        }

        #[test]
        fn line_diagonal() {
            let mut canvas = Canvas::new([10, 10]);
            let line = Line::new((0, 0), (9, 9));
            for p in line.iter() {
                canvas.put(p, '*');
            }
            canvas.print();
        }

        #[test]
        fn line_multi() {
            let mut canvas = Canvas::new([11, 7]);
            let lines = [
                Line::new((5, 3), (10, 5)),
                Line::new((5, 3), (10, 1)),
                Line::new((5, 3), (0, 1)),
                Line::new((5, 3), (0, 5)),
            ];
            for p in lines.iter().flat_map(|l| l.iter()) {
                canvas.put(p, '*');
            }
            canvas.print();
        }
    }

    #[cfg(test)]
    mod bresenham {
        use crate::prelude::*;

        #[test]
        fn line_diagonal() {
            let mut canvas = Canvas::new([10, 10]);
            let line = Bresenham::new((0, 0), (9, 9));
            for p in line {
                canvas.put(p, '*');
            }
            canvas.print();
        }
    }

    #[cfg(test)]
    mod bresenham_inclusive {
        use crate::prelude::*;

        #[test]
        fn line_diagonal() {
            let mut canvas = Canvas::new([10, 10]);
            let line = BresenhamInclusive::new((0, 0), (9, 9));
            for p in line {
                canvas.put(p, '*');
            }
            canvas.print();
        }
    }
}
