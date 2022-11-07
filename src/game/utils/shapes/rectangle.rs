use crate::game::prelude::*;

pub struct Rectangle {
    pub min: IVec2,
    pub max: IVec2,
}

impl Rectangle {
    pub fn new(point1: IVec2, point2: IVec2) -> Self {
        Self { min: point1.min(point2), max: point1.max(point2) }
    }
}

impl Default for Rectangle {
    fn default() -> Self { Self::new(IVec2::ZERO, IVec2::ONE) }
}
