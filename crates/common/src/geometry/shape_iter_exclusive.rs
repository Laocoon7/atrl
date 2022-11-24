pub trait ShapeIterExclusive {
    type ExlusiveIterator;
    /// returns an iterator over all points in the shape, exlusively
    fn iter_exlusive(&self) -> Self::ExlusiveIterator;
}
