use crate::prelude::*;
use std::{iter::Zip, ops::Index};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitGrid {
    pub size: UVec2,
    pub cells: BitVec<usize>,
}

impl BitGrid {
    #[inline(always)]
    pub fn new_fn<F>(size: impl Size2d, mut f: F) -> Self
    where
        F: FnMut(IVec2) -> bool,
    {
        let count = size.count();
        let mut cells = BitVec::with_capacity(count);
        for coord in size.iter() {
            cells.push(f(coord));
        }
        Self { size: size.as_uvec2(), cells }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.size.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.size.height()
    }

    #[inline]
    pub const fn size(&self) -> UVec2 {
        self.size
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Tests whether a point is in bounds.
    pub fn in_bounds(&self, point: impl Point2d) -> bool {
        point.is_valid(self.size())
    }

    /// Gets the index corresponding to a coordinate, which is row-wise.
    pub fn get_idx_unchecked(&self, point: impl Point2d) -> usize {
        point.as_index(self.width() as usize)
    }

    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get_idx(&self, coord: impl Point2d) -> Option<usize> {
        if coord.is_valid(self.size()) {
            Some(self.get_idx_unchecked(coord))
        } else {
            None
        }
    }

    /// Gets the `GridPoint` corresponding to an index
    pub fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    /// Try Gets the `GridPoint` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) {
            Some(pt)
        } else {
            None
        }
    }

    pub fn get(&self, index: impl Point2d) -> Option<&bool> {
        self.get_idx(index).map(|idx| &self.cells[idx])
    }

    pub fn get_mut(&mut self, index: impl Point2d) -> Option<BitRef<'_, bitvec::ptr::Mut>> {
        let width = self.width() as usize;
        self.cells.get_mut(index.as_index(width))
    }

    pub fn get_unchecked(&self, index: impl Point2d) -> &bool {
        self.cells.index(self.get_idx_unchecked(index))
    }

    /// Gets the `GridPoint` corresponding to an index
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the index is out of bounds.
    pub unsafe fn get_mut_unchecked(
        &mut self,
        index: impl Point2d,
    ) -> BitRef<'_, bitvec::ptr::Mut> {
        let w = self.width() as usize;
        self.cells.get_unchecked_mut(index.as_index(w))
    }

    pub fn set(&mut self, index: impl Point2d, value: bool) {
        if index.is_valid(self.size()) {
            let index = self.get_idx_unchecked(index);
            self.cells.set(index, value);
        }
    }

    pub fn set_unchecked(&mut self, index: impl Point2d, value: bool) {
        let index = self.get_idx_unchecked(index);
        self.cells.set(index, value);
    }

    ///////////////////////////////////////////////////////////////////////////
    // Iterator Functionality
    ///////////////////////////////////////////////////////////////////////////

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> bitvec::slice::Iter<'_, usize, bitvec::order::Lsb0> {
        self.cells.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> bitvec::slice::IterMut<'_, usize, bitvec::order::Lsb0> {
        self.cells.iter_mut()
    }

    pub fn point_iter(&self) -> PointIterRowMajor {
        self.size.iter()
    }

    pub fn enumerate(
        &self,
    ) -> Zip<PointIterRowMajor, bitvec::slice::Iter<'_, usize, bitvec::order::Lsb0>> {
        self.point_iter().zip(self.iter())
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl Index<usize> for BitGrid {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &bool {
        &self.cells[index]
    }
}

impl<P: Point2d> Index<P> for BitGrid {
    type Output = bool;

    #[inline]
    fn index(&self, index: P) -> &bool {
        self.get_unchecked(index)
    }
}
