use crate::prelude::*;

pub type VisibilityMap2d = Grid<u8>;

impl BitPacker for VisibilityMap2d {
    fn new_packer(size: impl Size2d) -> Grid<u8> {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Self::new_default([width, size.height()])
    }

    fn get_bit_at(&self, p: impl Point2d) -> bool {
        self.get((p.x() / 8, p.y())).map_or(false, |byte| (*byte >> (p.x() % 8)) & 0b0000_0001 == 1)
    }

    fn set_bit_at(&mut self, p: impl Point2d) {
        if let Some(byte) = self.get((p.x() / 8, p.y())) {
            self.set((p.x() / 8, p.y()), *byte | (1 << ((p.x().max(0)) % 8)) as u8);
        }
    }

    fn clear_bit_at(&mut self, p: impl Point2d) {
        if let Some(byte) = self.get((p.x() / 8, p.y())) {
            self.set((p.x() / 8, p.y()), *byte & !(1 << ((p.x().max(0)) % 8)));
        }
    }
}

impl VisibilityMap for VisibilityMap2d {
    fn get_visible(&self, p: impl Point2d) -> bool {
        self.get_bit_at(p)
    }

    fn set_visible(&mut self, p: impl Point2d) {
        self.set_bit_at(p);
    }
}
