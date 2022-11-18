pub trait Intersects<T> {
    /// returns true if `shape` intersects this shape
    fn intersects(&self, shape: T) -> bool;
}
