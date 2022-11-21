use ordered_float::OrderedFloat;

use crate::prelude::*;

pub trait PathMap {
    type ExitIterator: Iterator<Item = (IVec2, OrderedFloat<f32>)>;

    /// The cost of moving to a given node
    fn cost(&self, node: impl Point2d, movement_component: &Movement) -> OrderedFloat<f32>;

    /// The distance between two node points.
    fn distance(&self, a_node: impl Point2d, b_node: impl Point2d) -> OrderedFloat<f32>;

    /// Returns an iterator of the valid list of successors for a given node
    fn successors(&self, node: impl Point2d, movement_component: &Movement) -> Self::ExitIterator;
}
