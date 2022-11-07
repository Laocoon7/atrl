use super::Size2d;
use bevy::prelude::{IVec2, UVec2};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub size: UVec2,
    pub cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new_clone(size: impl Size2d, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, size: size.as_uvec2() }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new_copy(size: impl Size2d, value: T) -> Self {
        let count = size.count();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells, size: size.as_uvec2() }
    }
}

impl<T: Default> Grid<T> {
    pub fn new_default(size: impl Size2d) -> Self {
        let count = size.count();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self { cells, size: size.as_uvec2() }
    }
}

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
}
