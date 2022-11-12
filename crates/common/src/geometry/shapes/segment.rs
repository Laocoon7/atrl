use crate::prelude::*;

pub struct Segment {
    pub point1: IVec2,
    pub point2: IVec2,
}

impl Segment {
    pub fn new(point1: IVec2, point2: IVec2) -> Self { Self { point1, point2 } }
}

impl Default for Segment {
    fn default() -> Self { Self::new(IVec2::ZERO, IVec2::new(1, 0)) }
}
