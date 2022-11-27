use crate::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Triangle {
    start: IVec2,
    point1: IVec2,
    point2: IVec2,
}
impl Triangle {
    pub fn new(start: impl Point2d, point1: impl Point2d, point2: impl Point2d) -> Self {
        Self {
            start: start.as_ivec2(),
            point1: point1.as_ivec2(),
            point2: point2.as_ivec2(),
        }
    }
}

impl GridShape for Triangle {
    // TODO: check for bugs, this is just a quick implementation I thought of.
    fn get_points(&self) -> HashSet<IVec2> {
        let mut discovered = HashSet::new();
        for end_point in Line::new(self.point1, self.point2).get_points() {
            for current_point in Line::new(self.start, end_point) {
                discovered.insert(current_point);
            }
        }
        discovered
    }

    fn contains(&self, point: impl Point2d) -> bool { self.get_points().contains(&point.as_ivec2()) }

    fn get_count(&self) -> usize { self.get_points().len() }
}

impl GridShapeWithBorder for Triangle {
    fn get_border_points(&self) -> HashSet<IVec2> {
        let mut discovered = HashSet::new();
        for point in Line::new(self.start, self.point1) {
            discovered.insert(point);
        }
        for point in Line::new(self.point1, self.point2) {
            discovered.insert(point);
        }
        for point in Line::new(self.point2, self.start) {
            discovered.insert(point);
        }
        discovered
    }

    fn border_contains(&self, point: impl Point2d) -> bool {
        self.get_border_points().contains(&point.as_ivec2())
    }

    fn get_border_count(&self) -> usize { self.get_border_points().len() }
}
