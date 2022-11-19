use crate::prelude::*;

/// Bits2 is u8 in size, but only
/// the last two bits are used.
pub type Bits = u8;
/// packs values into a grid
/// each value uses 2 bits of room
/// so one u8 can hold 4 values
pub trait GridPacker {
    /// creates a smaller grid than asked for
    /// use size_packed() to get the number of
    /// values stored.
    /// NOTE: if `size.width()` is not perfectly
    /// divisible by 4, a buffer will be added.
    fn new_packer(size: impl Size2d) -> Grid<u8>;

    /// gets the number of values stored.
    /// should be used to iterate over the packed
    /// values.
    /// NOTE: if the original `size.width()` was not
    /// divisible by 4, x will contain `size.width() % 4`
    /// extra values.
    fn size_packed(&self) -> UVec2;

    /// returns if the `Point2d` is valid inside the bounds
    /// NOTE: if the origianl `size.width()` was not
    /// divisible by 4, points outside the intended
    /// area may be valid, and considered "in bounds"
    fn in_bounds_packed(&self, p: impl Point2d) -> bool;

    /// returns the value packed at `p`
    fn get_bits_at(&self, p: impl Point2d) -> Bits;

    /// sets the value packed at `p`
    fn set_bits_at(&mut self, p: impl Point2d, bits: Bits);
}
