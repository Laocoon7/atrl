use crate::prelude::*;

pub struct Circle {
    pub center: IVec2,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: IVec2, radius: i32) -> Self {
        Self { center, radius }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(IVec2::ZERO, 1)
    }
}
