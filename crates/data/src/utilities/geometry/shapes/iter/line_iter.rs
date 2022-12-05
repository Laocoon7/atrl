use crate::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Octant(u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    #[inline]
    fn from_points(start: (i64, i64), end: (i64, i64)) -> Self {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;
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
    fn to_octant(&self, p: (i64, i64)) -> (i64, i64) {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (p.1, -p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (-p.1, p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_octant(&self, p: (i64, i64)) -> (i64, i64) {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (-p.1, p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (p.1, -p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }
}

/// Line-drawing iterator
#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    x: i64,
    y: i64,
    z: i32,
    layer: u32,
    delta_x: i64,
    delta_y: i64,
    x1: i64,
    diff: i64,
    octant: Octant,
}

impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        let (s_abs_x, s_abs_y, s_abs_z) = start.absolute_position();
        let (e_abs_x, e_abs_y, _e_abs_z) = end.absolute_position();

        let octant = Octant::from_points((s_abs_x, s_abs_y), (e_abs_x, e_abs_y));
        let start_octant = octant.to_octant((s_abs_x, s_abs_y));
        let end_octant = octant.to_octant((e_abs_x, e_abs_y));

        let delta_x = end_octant.0 - start_octant.0;
        let delta_y = end_octant.1 - start_octant.1;

        Self {
            x: start_octant.0,
            y: start_octant.1,
            z: s_abs_z,
            layer: start.layer(),
            delta_x,
            delta_y,
            x1: end_octant.0,
            diff: delta_y - delta_x,
            octant,
        }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position {
        let current_point = (self.x, self.y);
        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.delta_x;
        }
        self.diff += self.delta_y;
        // loop inc
        self.x += 1;
        let (x, y) = self.octant.from_octant(current_point);

        Position::from_absolute_position((x, y, self.z), self.layer)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Position;

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
    pub fn new(start: Position, end: Position) -> Self { Self(BresenhamLineIter::new(start, end)) }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position { self.0.advance() }
}

impl Iterator for BresenhamLineInclusiveIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.x > self.0.x1 {
            None
        } else {
            Some(self.0.advance())
        }
    }
}

//#[cfg(test)]
// mod tests {
//    #[cfg(test)]
//    mod line {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_vertical() {
//            let mut canvas = Canvas::new([11, 11]);
//            let line = Line::new((0, 0), (0, 10));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_horizontal() {
//            let mut canvas = Canvas::new([11, 11]);
//            let line = Line::new((0, 0), (10, 0));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = Line::new((0, 0), (9, 9));
//            for p in line.iter() {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//
//        #[test]
//        fn line_multi() {
//            let mut canvas = Canvas::new([11, 7]);
//            let lines = [
//                Line::new((5, 3), (10, 5)),
//                Line::new((5, 3), (10, 1)),
//                Line::new((5, 3), (0, 1)),
//                Line::new((5, 3), (0, 5)),
//            ];
//            for p in lines.iter().flat_map(|l| l.iter()) {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//
//    #[cfg(test)]
//    mod bresenham {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = BresenhamLineIter::new((0, 0), (9, 9));
//            for p in line {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//
//    #[cfg(test)]
//    mod bresenham_inclusive {
//        use crate::prelude::*;
//
//        #[test]
//        fn line_diagonal() {
//            let mut canvas = Canvas::new([10, 10]);
//            let line = BresenhamLineInclusiveIter::new((0, 0), (9, 9));
//            for p in line {
//                canvas.put(p, '*');
//            }
//            canvas.print();
//        }
//    }
//}
