use crate::internal::*;
use crate::prelude::*;

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

    /// Convert dimensions to `UVec2` (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 { self.as_ivec2().as_uvec2() }

    /// Convert dimensions to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] { self.as_ivec2().to_array() }

    /// Convert dimensions to `IVec2` (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    /// Convert dimensions to `Vec2` (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 { self.as_ivec2().as_vec2() }

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
