use crate::game::prelude::*;

pub struct Ray {
    pub origin: IVec2,
    pub direction: IVec2,
}

impl Ray {
    pub fn new(origin: IVec2, direction: IVec2) -> Self { Self { origin, direction } }
}

impl Default for Ray {
    fn default() -> Self { Self::new(IVec2::ZERO, IVec2::new(1, 0)) }
}
