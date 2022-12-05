use crate::prelude::*;

//////////////////////////////////////////////////////////////////////////////////////////
/// Bresenham Algo
//////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Octant(u8);

/// Line-drawing iterator
#[derive(Debug, Clone)]
pub struct BresenhamLineIter {
    dx: i32,
    dy: i32,
    x1: u32,
    diff: i32,
    octant: Octant,
    pos: Position,
}

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    #[inline]
    fn from_points(start: Position, end: Position) -> Self {
        let mut dx = (end.x() - start.x()) as i32;
        let mut dy = (end.y() - start.y()) as i32;
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
    fn to_octant(&self, p: Position) -> Position {
        let IVec2 { x, y } = p.gridpoint().as_ivec2();
        let new_lp = match self.0 {
            0 => IVec2::new(x, y),
            1 => IVec2::new(y, x),
            2 => IVec2::new(y, -x),
            3 => IVec2::new(-x, y),
            4 => IVec2::new(-x, -y),
            5 => IVec2::new(-y, -x),
            6 => IVec2::new(-y, x),
            7 => IVec2::new(x, -y),
            _ => unreachable!(),
        };

        p.set_xy(new_lp.as_uvec2());
        p
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_octant(&self, p: Position) -> Position {
        let IVec2 { x, y } = p.gridpoint().as_ivec2();
        let new_lp = match self.0 {
            0 => IVec2::new(x, y),
            1 => IVec2::new(y, x),
            2 => IVec2::new(-y, x),
            3 => IVec2::new(-x, y),
            4 => IVec2::new(-x, -y),
            5 => IVec2::new(-y, -x),
            6 => IVec2::new(y, -x),
            7 => IVec2::new(x, -y),
            _ => unreachable!(),
        };

        p.set_xy(new_lp.as_uvec2());
        p
    }
}

impl BresenhamLineIter {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Position, end: Position) -> Self {
        let octant = Octant::from_points(start, end);
        let start = octant.to_octant(start);
        let end = octant.to_octant(end);
        let dx = (end.x() - start.x()) as i32;
        let dy = (end.y() - start.y()) as i32;

        Self {
            dx,
            dy,
            octant,
            pos: start,
            x1: end.x(),
            diff: dy - dx,
        }
    }

    /// Return the next point without checking if we are past `end`.
    #[inline]
    pub fn advance(&mut self) -> Position {
        if self.diff >= 0 {
            self.diff -= self.dx;
            self.pos.set_y(self.pos.y() + 1)
        }
        self.diff += self.dy;
        // loop inc
        self.pos.set_x(self.pos.x() + 1);
        self.octant.from_octant(self.pos)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = Position;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x() >= self.x1 {
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
        if self.0.pos.x() > self.0.x1 {
            None
        } else {
            Some(self.0.advance())
        }
    }
}
