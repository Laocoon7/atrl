use crate::internal::*;

pub struct Polygon {
    pub points: Vec<IVec2>,
}

#[allow(dead_code)]
impl Polygon {
    pub fn new(points: Vec<IVec2>) -> Self { Self { points } }
}
