use crate::game::prelude::*;


pub struct Triangle {
    pub point1: IVec2,
    pub point2: IVec2,
    pub point3: IVec2,
}

impl Triangle {
    pub fn new(point1: IVec2, point2: IVec2, point3: IVec2) -> Self {
        Self {
            point1,
            point2,
            point3,
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new(IVec2::ZERO, IVec2::new(1, 0), IVec2::new(0, 1))
    }
}