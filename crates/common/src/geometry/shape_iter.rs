pub trait ShapeIter {
    type Iterator;
    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}
