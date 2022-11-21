use crate::prelude::*;

pub trait BitPacker {
    fn new_packer(size: impl Size2d) -> Grid<u8>;
    fn new_packer_with(size: impl Size2d, value: u8) -> Grid<u8>;
    fn new_packer_fn<F>(size: impl Size2d, f: F) -> Grid<u8>
    where
        F: FnMut(IVec2) -> u8;

    fn get_bit_at(&self, p: impl Point2d) -> bool;
    fn set_bit_at(&mut self, p: impl Point2d);
    fn clear_bit_at(&mut self, p: impl Point2d);
}
