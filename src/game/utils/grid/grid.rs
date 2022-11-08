use crate::game::prelude::*;
use std::{
    ops::{Index, IndexMut},
    slice,
};

#[allow(dead_code)]
pub type PointIter = PointIterRowMajor;
pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridRows<'a, T> = slice::Chunks<'a, T>;
pub type GridRowsMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub size: UVec2,
    pub cells: Vec<T>,
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
    pub fn new_clone(size: impl Size2d, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, size: size.as_uvec2() }
    }
}

#[allow(dead_code)]
impl<T: Copy> Grid<T> {
    pub fn new_copy(size: impl Size2d, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells, size: size.as_uvec2() }
    }
}

#[allow(dead_code)]
impl<T: Default> Grid<T> {
    pub fn new_default(size: impl Size2d) -> Self {
        let count = size.count();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self { cells, size: size.as_uvec2() }
    }
}

#[allow(dead_code)]
impl<T> Grid<T> {
    #[inline(always)]
    pub fn new_fn<F>(size: impl Size2d, mut f: F) -> Self
    where
        F: FnMut(IVec2) -> T,
    {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        for coord in size.iter() {
            cells.push(f(coord));
        }
        Self { size: size.as_uvec2(), cells }
    }

    #[inline]
    pub fn width(&self) -> u32 { self.size.width() }

    #[inline]
    pub fn height(&self) -> u32 { self.size.height() }

    #[inline]
    pub fn size(&self) -> UVec2 { self.size }

    #[inline]
    pub fn len(&self) -> usize { self.cells.len() }

    #[inline]
    pub fn is_empty(&self) -> bool { self.cells.is_empty() }

    /// Tests whether a point is in bounds.
    pub fn in_bounds(&self, point: impl Point2d) -> bool { point.is_valid(self.size()) }

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

    pub fn get(&self, index: impl Point2d) -> Option<&T> {
        self.get_idx(index).map(|idx| &self.cells[idx])
    }

    pub fn get_mut(&mut self, index: impl Point2d) -> Option<&mut T> {
        self.get_idx(index).map(move |idx| &mut self.cells[idx])
    }

    pub fn get_unchecked(&self, index: impl Point2d) -> &T {
        self.cells.index(self.get_idx_unchecked(index))
    }

    pub fn get_mut_unchecked(&mut self, index: impl Point2d) -> &mut T {
        self.cells.index_mut(self.get_idx_unchecked(index))
    }

    pub fn set(&mut self, index: impl Point2d, value: T) -> Option<T> {
        if index.is_valid(self.size()) {
            let index = self.get_idx_unchecked(index);
            Some(std::mem::replace(&mut self.cells[index], value))
        } else {
            None
        }
    }

    pub fn set_unchecked(&mut self, index: impl Point2d, value: T) -> T {
        let index = self.get_idx_unchecked(index);
        std::mem::replace(&mut self.cells[index], value)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Iterator Functionality
    ///////////////////////////////////////////////////////////////////////////

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> GridIter<T> { self.cells.iter() }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> GridIterMut<T> { self.cells.iter_mut() }

    pub fn point_iter(&self) -> PointIterRowMajor { self.size.iter() }

    #[inline]
    pub fn rows(&self) -> GridRows<T> { self.cells.chunks(self.size.width() as usize) }

    #[inline]
    pub fn rows_mut(&mut self) -> GridRowsMut<T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }

    #[inline]
    pub fn cols(&self) -> GridRows<T> { self.cells.chunks(self.size.width() as usize) }

    #[inline]
    pub fn cols_mut(&mut self) -> GridRowsMut<T> {
        self.cells.chunks_mut(self.size.width() as usize)
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl<T: Copy> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T { &self.cells[index] }
}

impl<T: Copy> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.cells[index] }
}

impl<T: Copy, P: Point2d> Index<P> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &T { self.get_unchecked(index) }
}

impl<T: Copy, P: Point2d> IndexMut<P> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_unchecked(index) }
}
