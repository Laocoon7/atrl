use crate::game::prelude::*;

////////////////////////////////////////////////////////////
// Point2d
////////////////////////////////////////////////////////////

/// A trait for types representing a 2d Point.
pub trait Point2d: Clone + Copy {
    #[allow(clippy::new_ret_no_self)]
    /// Construct a IVec2 (impl of GridPoint)
    fn new(x: i32, y: i32) -> IVec2 { IVec2::new(x, y) }

    /// Returns x coordinate.
    fn x(&self) -> i32;

    /// Returns y coordinate.
    fn y(&self) -> i32;

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 { self.as_ivec2().as_uvec2() }

    /// Convert dimensions to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] { self.as_ivec2().to_array() }

    /// Convert dimensions to IVec2 (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    /// Get the grid point's corresponding 1d index.
    #[inline(always)]
    fn as_index(&self, grid_width: usize) -> usize {
        self.y() as usize * grid_width + self.x() as usize
    }

    /// Returns true if the point is valid for the given size.
    #[inline]
    fn is_valid<S>(&self, size: S) -> bool
    where
        S: Size2d,
    {
        let x = self.x();
        let y = self.y();

        x >= 0 && y >= 0 && x < size.width() as i32 && y < size.height() as i32
    }
}

#[macro_export]
macro_rules! impl_grid_point_array {
    ($type:ty) => {
        impl Point2d for $type {
            fn x(&self) -> i32 { self[0] as i32 }

            fn y(&self) -> i32 { self[1] as i32 }
        }
    };
}

#[macro_export]
macro_rules! impl_grid_point_tuple {
    ($type:ty) => {
        impl Point2d for $type {
            fn x(&self) -> i32 { self.0 as i32 }

            fn y(&self) -> i32 { self.1 as i32 }
        }
    };
}

impl_grid_point_array!(IVec2);
impl_grid_point_array!(UVec2);
impl_grid_point_tuple!((u32, u32));
impl_grid_point_tuple!((i32, i32));
impl_grid_point_tuple!((usize, usize));

////////////////////////////////////////////////////////////
// Size2d
////////////////////////////////////////////////////////////

/// A trait for types representing a 2d size.
pub trait Size2d: Clone + Copy {
    /// Returns width coordinate.
    fn width(&self) -> u32;

    /// Returns height coordinate.
    fn height(&self) -> u32;

    #[inline]
    fn count(&self) -> usize { (self.width() * self.height()) as usize }

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 { self.as_ivec2().as_uvec2() }

    /// Convert dimensions to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] { self.as_ivec2().to_array() }

    /// Convert dimensions to IVec2 (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 { IVec2::new(self.width() as i32, self.height() as i32) }

    #[inline]
    fn point_in_bounds<P>(&self, point: P) -> bool
    where
        P: Point2d,
    {
        point.x() >= 0
            && point.y() >= 0
            && point.x() < self.width() as i32
            && point.y() < self.height() as i32
    }

    /// Returns an iterator over all points in the grid.
    fn iter(self) -> PointIterRowMajor { PointIterRowMajor::new(self) }
}

////////////////////////////////////////////////////////////
// Point Iter
////////////////////////////////////////////////////////////

pub struct PointIterRowMajor {
    coord: IVec2,
    size: UVec2,
}

impl PointIterRowMajor {
    pub fn new(size: impl Size2d) -> Self {
        Self { size: size.as_uvec2(), coord: IVec2::new(0, 0) }
    }
}

impl Iterator for PointIterRowMajor {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.height() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;

        if self.coord.x == self.size.width() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }

        Some(coord)
    }
}

////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! impl_size2d_array {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 { self[0] as u32 }

            fn height(&self) -> u32 { self[1] as u32 }
        }
    };
}

#[macro_export]
macro_rules! impl_size2d_tuple {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 { self.0 as u32 }

            fn height(&self) -> u32 { self.1 as u32 }
        }
    };
}

impl_size2d_array!(IVec2);
impl_size2d_array!(UVec2);
impl_size2d_array!([u32; 2]);
impl_size2d_array!([i32; 2]);
impl_size2d_array!([usize; 2]);