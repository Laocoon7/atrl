use std::{
    ops::{Index, IndexMut},
    slice,
};

use crate::prelude::*;
pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridChunks<'a, T> = slice::Chunks<'a, T>;
pub type GridChunksMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T: GridParam> {
    pub size: UVec2,
    pub cells: Vec<T>,
}

// Grid Layer
impl<T: GridParam> GridLayer<T> for Grid<T> {
    type MutableReturn<'a> = &'a mut T;

    #[inline(always)]
    fn new_clone(size: impl Size2d, value: T) -> Self
    where T: Clone {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self {
            cells,
            size: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: impl GridPoint, source: &Self, from: impl GridPoint, size: impl Size2d)
    where T: Clone {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x() as u32, y + from.y() as u32)) {
                    self.set((x + to.x() as u32, y + to.y() as u32), val.clone());
                }
            }
        }
    }

    #[inline(always)]
    fn new_copy(size: impl Size2d, value: T) -> Self
    where T: Copy {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self {
            cells,
            size: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: impl GridPoint, source: &Self, from: impl GridPoint, size: impl Size2d)
    where T: Copy {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x() as u32, y + from.y() as u32)) {
                    self.set((x + to.x() as u32, y + to.y() as u32), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_default(size: impl Size2d) -> Self
    where T: Default {
        let count = size.count();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self {
            cells,
            size: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn new_fn(size: impl Size2d, f: impl Fn(IVec2) -> T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        for coord in size.iter() {
            cells.push(f(coord));
        }
        Self {
            size: size.as_uvec2(),
            cells,
        }
    }

    #[inline]
    fn width(&self) -> u32 { self.size.width() }

    #[inline]
    fn height(&self) -> u32 { self.size.height() }

    #[inline]
    fn size(&self) -> UVec2 { self.size }

    #[inline]
    fn len(&self) -> usize { self.cells.len() }

    #[inline]
    fn is_empty(&self) -> bool { self.cells.is_empty() }

    #[inline]
    fn in_bounds(&self, pos: impl GridPoint) -> bool { pos.is_valid(self.size()) }

    #[inline]
    fn get_idx(&self, pos: impl GridPoint) -> Option<usize> {
        if pos.is_valid(self.size()) {
            Some(self.get_idx_unchecked(pos))
        } else {
            None
        }
    }

    #[inline]
    fn get_idx_unchecked(&self, point: impl GridPoint) -> usize { point.as_index_unchecked(self.width()) }

    #[inline]
    fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) {
            Some(pt)
        } else {
            None
        }
    }

    #[inline]
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    #[inline]
    fn get(&self, index: impl GridPoint) -> Option<&T> { self.get_idx(index).map(|idx| &self.cells[idx]) }

    #[inline]
    fn get_mut(&mut self, index: impl GridPoint) -> Option<&mut T> {
        self.get_idx(index).map(move |idx| &mut self.cells[idx])
    }

    #[inline]
    fn get_unchecked(&self, index: impl GridPoint) -> &T { self.cells.index(self.get_idx_unchecked(index)) }

    #[inline]
    fn get_mut_unchecked(&mut self, index: impl GridPoint) -> &mut T {
        self.cells.index_mut(self.get_idx_unchecked(index))
    }

    #[inline]
    fn set(&mut self, index: impl GridPoint, value: T) -> Option<T> {
        if index.is_valid(self.size()) {
            let index = self.get_idx_unchecked(index);
            Some(std::mem::replace(&mut self.cells[index], value))
        } else {
            None
        }
    }

    #[inline]
    fn set_unchecked(&mut self, index: impl GridPoint, value: T) -> T {
        let index = self.get_idx_unchecked(index);
        std::mem::replace(&mut self.cells[index], value)
    }
}

impl<T: GridParam> GridIterable<T> for Grid<T> {
    type IterChunkMutReturn<'a> = GridChunksMut<'a, T>;
    type IterChunkReturn<'a> = GridChunks<'a, T>;
    type IterMutReturn<'a> = GridIterMut<'a, T>;
    type IterReturn<'a> = GridIter<'a, T>;

    #[inline]
    fn iter(&self) -> GridIter<T> { self.cells.iter() }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    fn iter_mut(&mut self) -> GridIterMut<T> { self.cells.iter_mut() }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor { self.size.iter() }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> { self.point_iter().zip(self.iter()) }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.size.width() as usize) }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.size.width() as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.size.width() as usize) }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.size.width() as usize)
    }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < self.size().count() {
            let w = self.width() as usize;
            return Some(self.cells[x..].iter().step_by(w));
        } else {
            None
        }
    }

    #[inline]
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<Self::IterReturn<'_>> {
        let w = self.width() as usize;
        return self.cells[x..].iter().step_by(w);
    }
}

///////////////////////////////////////////////////////////////////////////
// Deref/DerefMut
///////////////////////////////////////////////////////////////////////////
// Deref
impl<T: GridParam> std::ops::Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.cells }
}

// DerefMut
impl<T: GridParam> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cells }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////
impl<T: Copy + GridParam> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T { &self.cells[index] }
}
impl<T: Copy + GridParam> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.cells[index] }
}

impl<T: Copy + GridParam, P: GridPoint> Index<P> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &T { self.get_unchecked(index) }
}

impl<T: Copy + GridParam, P: GridPoint> IndexMut<P> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_unchecked(index) }
}
