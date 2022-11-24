use super::grid_shape::*;
use super::line::Line;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: IVec2,
    radius: u32,
}

impl Circle {
    pub fn new<R: Into<u32>>(center: impl Point2d, radius: R) -> Self {
        Self { center: center.as_ivec2(), radius: radius.into() }
    }
}

impl GridShape for Circle {
    fn get_count(&self) -> usize {
        self.get_points().len()
    }

    fn contains(&self, point: impl Point2d) -> bool {
        self.get_points().contains(&point.as_ivec2())
    }

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
