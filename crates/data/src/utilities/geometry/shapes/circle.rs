use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: Position,
    radius: u32,
}

impl Circle {
    pub fn new<R: Into<u32>>(center: Position, radius: R) -> Self {
        Self {
            center: center,
            radius: radius.into(),
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
    pub fn as_horizontal_line(&self) -> Line {
        Line::new((self.left(), self.center.y), (self.right(), self.center.y))
    }

    #[inline]
    pub fn as_vertical_line(&self) -> Line {
        Line::new((self.center.x, self.bottom()), (self.center.x, self.top()))
    }
}

impl Shape for Circle {
    #[inline]
    // FIX: PERF
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }

    #[inline]
    fn contains(&self, position: Position) -> bool { self.get_positions().contains(&position) }

    #[inline]
    fn get_positions(&self) -> HashSet<Position> {
        let mut discovered = HashSet::new();
        let mut d = (5 - (self.radius as i32 * 4)) / 4;
        let mut x = 0;
        let mut y = self.radius as i32;

        loop {
            let line = Line::new(
                (self.center.x + x, self.center.y + y),
                (self.center.x + x, self.center.y - y),
            );
            for point in line.get_positions() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x - x, self.center.y + y),
                (self.center.x - x, self.center.y - y),
            );
            for point in line.get_positions() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x + y, self.center.y + x),
                (self.center.x + y, self.center.y - x),
            );
            for point in line.get_positions() {
                discovered.insert(point);
            }

            let line = Line::new(
                (self.center.x - y, self.center.y + x),
                (self.center.x - y, self.center.y - x),
            );
            for point in line.get_positions() {
                discovered.insert(point);
            }

            //// Circumfrence
            // discovered.insert(IVec2::new(self.center.x + x, self.center.y + y));
            // discovered.insert(IVec2::new(self.center.x + x, self.center.y - y));
            // discovered.insert(IVec2::new(self.center.x - x, self.center.y + y));
            // discovered.insert(IVec2::new(self.center.x - x, self.center.y - y));
            // discovered.insert(IVec2::new(self.center.x + y, self.center.y + x));
            // discovered.insert(IVec2::new(self.center.x + y, self.center.y - x));
            // discovered.insert(IVec2::new(self.center.x - y, self.center.y + x));
            // discovered.insert(IVec2::new(self.center.x - y, self.center.y - x));

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
