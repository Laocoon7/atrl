use super::astar;
use crate::prelude::*;

pub enum PathFinder {
    Astar,
}

impl PathFinder {
    pub fn compute(
        &self,
        start: impl Point2d,
        end: impl Point2d,
        movement_component: &Movement,
        path_map: &impl PathMap,
    ) -> Option<(Vec<IVec2>, OrderedFloat<f32>)> {
        match self {
            Self::Astar => astar(start, end, movement_component, path_map),
        }
    }
}
