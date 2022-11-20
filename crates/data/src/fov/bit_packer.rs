use crate::prelude::*;

pub trait BitPacker {
    fn new_packer(size: impl Size2d) -> Grid<u8>;
    fn get_bit_at(&self, p: impl Point2d) -> bool;
    fn set_bit_at(&mut self, p: impl Point2d);
    fn clear_bit_at(&mut self, p: impl Point2d);
}
