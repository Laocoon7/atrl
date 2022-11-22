use crate::prelude::*;
use std::{iter, slice};

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridEnumerate<'a, T> = iter::Zip<PointIterRowMajor, slice::Iter<'a, T>>;

pub type GridRows<'a, T> = slice::Chunks<'a, T>;
pub type GridRowsMut<'a, T> = slice::ChunksMut<'a, T>;
pub type GridIterCol<'a, T> = iter::StepBy<slice::Iter<'a, T>>;

pub trait GridIterable<T: GridParam> {
    ///Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    fn iter(&self) -> GridIter<T>;

    ///Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    fn iter_mut(&mut self) -> GridIterMut<T>;

    /// Returns an iterator for every point in the grid.
    fn point_iter(&self) -> PointIterRowMajor;

    /// Returns an iterator for every point in the grid with its corresponding point index
    fn enumerate(&self) -> GridEnumerate<T>;

    ///////////////////////////////////////////////////////////////////////////
    // Row / Column Iterators
    ///////////////////////////////////////////////////////////////////////////
    /// Returns an iterator over the rows of the grid.
    fn rows(&self) -> GridRows<T>;

    /// Returns a mutable iterator over the rows of the grid.
    fn rows_mut(&mut self) -> GridRowsMut<T>;

    /// Returns an iterator over the columns of the grid.
    fn cols(&self) -> GridRows<T>;

    /// Returns a mutable iterator over the columns of the grid.
    fn cols_mut(&mut self) -> GridRowsMut<T>;

    /// Returns an iterator over a column of the grid.
    fn iter_column(&self, x: usize) -> Option<GridIterCol<T>>;

    /// Returns a mutable iterator over a column of the grid.
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<T>;
}
