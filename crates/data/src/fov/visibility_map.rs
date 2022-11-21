use crate::prelude::*;

pub struct VisibilityMap {
    grid: Grid<u8>,
}

impl VisibilityMap {
    pub fn new(size: impl Size2d) -> Self {
        let mut width = size.width() / 8;
        if size.width() % 8 != 0 {
            width += 1;
        }

        Self { grid: Grid::new_default([width, size.height()]) }
    }

    fn get_byte_index(&self, position: IVec2) -> Option<(IVec2, u8)> {
        let byte_index = IVec2::new(position.x / 8, position.y);
        let bit_index = position.x % 8;
        if byte_index.is_valid(self.grid.size) && bit_index >= 0 {
            Some((byte_index, bit_index as u8))
        } else {
            None
        }
    }
}

impl FovReceiver for VisibilityMap {
    fn get_visible(&self, position: IVec2) -> bool {
        if let Some((byte_index, bit_index)) = self.get_byte_index(position) {
            let bit = (*self.grid.get_unchecked(byte_index) >> bit_index) & 0b0000_0001;
            bit == 1
        } else {
            false
        }
    }

    fn set_visible(&mut self, position: IVec2) {
        if let Some((byte_index, bit_index)) = self.get_byte_index(position) {
            println!("{}, {}, {}", position, byte_index, bit_index);
            // get old byte, and set the bit index
            let byte = *self.grid.get_unchecked(byte_index) | (1 << bit_index);

            self.grid.set_unchecked(byte_index, byte);
        }
    }
}
