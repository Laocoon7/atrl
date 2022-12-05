use crate::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Octant(u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    #[inline]
    const fn from_points(start: Position, end: Position) -> Self {
        let (start_x, start_y, _start_z) = start.absolute_position();
        let (end_x, end_y, _end_z) = end.absolute_position();

        let mut delta_x = end_x - start_x;
        let mut delta_y = end_y - start_y;

        let mut octant = 0;

        if delta_y < 0 {
            delta_x = -delta_x;
            delta_y = -delta_y;
            octant += 4;
        }
        if delta_x < 0 {
            let tmp = delta_x;
            delta_x = delta_y;
            delta_y = -tmp;
            octant += 2;
        }
        if delta_x < delta_y {
            octant += 1;
        }

        Self(octant)
    }

    #[inline]
    fn to_octant(&self, p: Position) -> Position {
        let (mut x, mut y, z) = p.absolute_position();

        (x, y) = match self.0 {
            0 => (x, y),
            1 => (y, x),
            2 => (y, -x),
            3 => (-x, y),
            4 => (-x, -y),
            5 => (-y, -x),
            6 => (-y, x),
            7 => (x, -y),
            _ => unreachable!(),
        };
        Position::from_absolute_position((x, y, z), p.layer())
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_octant(&self, p: Position) -> Position {
        let (mut x, mut y, z) = p.absolute_position();

        (x, y) = match self.0 {
            0 => (x, y),
            1 => (y, x),
            2 => (-y, x),
            3 => (-x, y),
            4 => (-x, -y),
            5 => (-y, -x),
            6 => (y, -x),
            7 => (x, -y),
            _ => unreachable!(),
        };
        Position::from_absolute_position((x, y, z), p.layer())
    }
}

/// Line-drawing iterator
#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    delta_x: i32,    // sub from step_ratio when step_ratio is 0/positive
    delta_y: i32,    // add to step_ratio when step_ratio is negative
    step_ratio: i32, // determines whether we need to move vertical or horizontal next
    end_x: i64,      // determins whether we have moved far enough
    pos: Position,   // our current position
    octant: Octant,  // the direction we are moving
}

impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        let end_x = end.absolute_x();
        let octant = Octant::from_points(start, end);
        let start_octant = octant.to_octant(start);
        let end_octant = octant.to_octant(end);

        let delta_x = end_octant.x() as i32 - start_octant.x() as i32;
        let delta_y = end_octant.y() as i32 - start_octant.y() as i32;

        Self {
            delta_x,
            delta_y,
            octant,
            pos: start_octant,
            end_x,
            step_ratio: delta_y - delta_x,
        }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position {
        let pos = self.pos;
        if self.step_ratio >= 0 {
            self.step_ratio -= self.delta_x;
            self.pos.set_y(self.pos.y() + 1);
        }
        self.step_ratio += self.delta_y;

        // loop inc
        self.pos.set_x(self.pos.x() + 1);
        self.octant.from_octant(pos)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let abs_x = self.pos.absolute_x();
        if abs_x >= self.end_x {
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
        let (abs_x, _abs_y, _abs_z) = self.0.pos.absolute_position();
        if abs_x > self.0.end_x {
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
            let p1 = Position::ZERO;
            let mut p2 = Position::ZERO;
            p2.set_xy(UVec2::new(0, 10));

            let mut canvas = Canvas::new([11, 11]);
            let line = Line::new(p1, p2);
            for p in line.iter() {
                canvas.put(p.gridpoint(), '*');
            }
            canvas.print();
        }
    }
}
