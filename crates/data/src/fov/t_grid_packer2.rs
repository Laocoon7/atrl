use crate::prelude::*;

/// Bits2 is u8 in size, but only
/// the last two bits are used.
pub type Bits2 = u8;
/// packs values into a grid
/// each value uses 2 bits of room
/// so one u8 can hold 4 values
pub trait GridPacker2 {
    /// creates a smaller grid than asked for
    /// use size_packed() to get the number of
    /// values stored.
    /// NOTE: if `size.width()` is not perfectly
    /// divisible by 4, a buffer will be added.
    fn new_packer_2(size: impl Size2d) -> Grid<u8>;
    /// gets the number of values stored.
    /// should be used to iterate over the packed
    /// values.
    /// NOTE: if the original `size.width()` was not
    /// divisible by 4, x will contain `size.width() % 4`
    /// extra values.
    fn size_packed(&self) -> UVec2;
    /// returns the value packed at `p`
    fn get_bits2_at(&self, p: impl Point2d) -> Bits2;
    /// sets the value packed at `p`
    fn set_bits2_at(&mut self, p: impl Point2d, bits: Bits2);
}
