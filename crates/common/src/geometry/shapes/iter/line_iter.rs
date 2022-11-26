use crate::prelude::*;
//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Octant(u8);
/// Line-drawing iterator
#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x1: i32,
    diff: i32,
    octant: Octant,
}
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
    #[allow(clippy::wrong_self_convention)]
    fn from_octant(&self, p: impl Point2d) -> IVec2 {
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
impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
        let octant = Octant::from_points(start, end);
        let start = octant.to_octant(start);
        let end = octant.to_octant(end);
        let dx = end.x() - start.x();
        let dy = end.y() - start.y();
        Self {
            x: start.x(),
            y: start.y(),
            dx,
            dy,
            x1: end.x(),
            diff: dy - dx,
            octant,
        }
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
        self.octant.from_octant(p)
    }
}
impl Iterator for BresenhamLineIter {
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
#[derive(Debug, Clone)]
pub struct BresenhamLineInclusiveIter(BresenhamLineIter);
impl BresenhamLineInclusiveIter {
    /// Creates a new iterator. Yields points `start..=end`.
    #[inline]
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self { Self(BresenhamLineIter::new(start, end)) }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> impl Point2d { self.0.advance() }
}

impl Iterator for BresenhamLineInclusiveIter {
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
