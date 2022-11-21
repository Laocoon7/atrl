use crate::prelude::*;

/// A trait used by the pathfinding algorithms to calculate the resulting path.
pub trait PathMapProvider {
    /// The cost of moving to a given node
    fn cost(&self, node: impl Point2d, movement_component: &Movement) -> OrderedFloat<f32>;
}
