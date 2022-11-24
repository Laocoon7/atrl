use super::shared::PathAlgorithm;
use crate::prelude::*;
pub enum PathFinder {
    Astar,
}

impl PathFinder {
    pub fn compute(
        &self,
        origin: impl Point2d,
        destination: impl Point2d,
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &impl PathProvider,
    ) -> Option<Vec<IVec2>> {
        let origin = origin.as_ivec2();
        let destination = destination.as_ivec2();
        match self {
            Self::Astar => AStar::compute_path(
                origin,
                destination,
                movement_type,
                partial_path_on_failure,
                provider,
            ),
        }
    }
}
