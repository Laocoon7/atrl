use crate::components::Position;

pub trait ShapeIter {
    type Iterator: Iterator<Item = Position>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}
