use crate::prelude::*;

pub type TVisibilityMap2d = Grid<TVisibilityFlag>;

impl GridPacker2 for TVisibilityMap2d {
    fn new_packer_2(size: impl Size2d) -> Grid<u8> {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Grid::new_default([width, size.height()])
    }

    fn size_packed(&self) -> UVec2 {
        let width = self.size.x * 4;
        UVec2::new(width, self.size.y)
    }

    fn get_bits2_at(&self, p: impl Point2d) -> Bits2 {
        if let Some(byte) = self.get((p.x() / 4, p.y())) {
            (*byte >> ((p.x() % 4) * 2)) & 0b0000_0011
        } else {
            0
        }
    }

    fn set_bits2_at(&mut self, p: impl Point2d, bits: Bits2) {
        if let Some(byte) = self.get((p.x() / 4, p.y())) {
            self.set((p.x() / 4, p.y()), !((!bits) << ((p.x() % 4) * 2)) & byte);
        }
    }
}

impl TVisibilityMapUtility for TVisibilityMap2d {
    fn get_visible(&self, p: impl Point2d) -> bool {
        (self.get_bits2_at(p) & VISIBLE) != 0
    }

    fn set_visible(&mut self, p: impl Point2d) {
        self.set_bits2_at(p, self.get_bits2_at(p) | VISIBLE);
    }

    fn clear_visible(&mut self, p: impl Point2d) {
        self.set_bits2_at(p, self.get_bits2_at(p) & !VISIBLE);
    }

    fn get_opaque(&self, p: impl Point2d) -> bool {
        (self.get_bits2_at(p) & OPAQUE) != 0
    }

    fn set_opaque(&mut self, p: impl Point2d) {
        self.set_bits2_at(p, self.get_bits2_at(p) | OPAQUE);
    }

    fn clear_opaque(&mut self, p: impl Point2d) {
        self.set_bits2_at(p, self.get_bits2_at(p) & !OPAQUE);
    }

    fn clear_all(&mut self) {
        self.iter_mut().for_each(|v| *v = 0)
    }

    fn clear_all_visible(&mut self) {
        let size = self.size_packed();
        for y in 0..size.y {
            for x in 0..size.x {
                self.clear_visible((x, y));
            }
        }
    }

    fn clear_all_opaque(&mut self) {
        let size = self.size_packed();
        for y in 0..size.y {
            for x in 0..size.x {
                self.clear_opaque((x, y));
            }
        }
    }
}
