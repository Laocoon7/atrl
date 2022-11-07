use crate::game::prelude::*;


pub struct Line {
    pub point1: IVec2,
    pub point2: IVec2,
}

impl Line {
    pub fn new(point1: IVec2, point2: IVec2) -> Self {
        Self {
            point1,
            point2,
        }
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new(IVec2::ZERO, IVec2::new(1, 0))
    }
}