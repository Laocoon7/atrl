use crate::prelude::*;

pub type VisibilityMap2d = Grid<VisibilityFlag>;

impl GridPacker for VisibilityMap2d {
    fn new_packer(size: impl Size2d) -> Grid<u8> {
        let mut width = size.width() / 4;
        if size.width() % 4 != 0 {
            width += 1;
        }

        Self::new_default([width, size.height()])
    }

    fn size_packed(&self) -> UVec2 {
        let width = self.size.x * 4;
        UVec2::new(width, self.size.y)
    }

    fn in_bounds_packed(&self, p: impl Point2d) -> bool {
        let size = self.size_packed();
        size.contains(p)
    }

    fn get_bits_at(&self, p: impl Point2d) -> Bits {
        self.get((p.x() / 4, p.y())).map_or(0, |byte| (*byte >> ((p.x() % 4) * 2)) & 0b0000_0011)
    }

    fn set_bits_at(&mut self, p: impl Point2d, bits: Bits) {
        if let Some(byte) = self.get((p.x() / 4, p.y())) {
            let mut byte = *byte;
            let pos = (p.x() % 4) * 2;
            byte &= !3 << pos;
            byte |= bits << pos;
            self.set((p.x() / 4, p.y()), byte);
        }
    }
}

impl VisibilityMap for VisibilityMap2d {
    fn get_visible(&self, p: impl Point2d) -> bool {
        (self.get_bits_at(p) & VISIBLE) != 0
    }

    fn set_visible(&mut self, p: impl Point2d) {
        self.set_bits_at(p, self.get_bits_at(p) | VISIBLE);
    }

    fn clear_visible(&mut self, p: impl Point2d) {
        self.set_bits_at(p, self.get_bits_at(p) & !VISIBLE);
    }

    fn get_opaque(&self, p: impl Point2d) -> bool {
        (self.get_bits_at(p) & OPAQUE) != 0
    }

    fn set_opaque(&mut self, p: impl Point2d) {
        self.set_bits_at(p, self.get_bits_at(p) | OPAQUE);
    }

    fn clear_opaque(&mut self, p: impl Point2d) {
        self.set_bits_at(p, self.get_bits_at(p) & !OPAQUE);
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
