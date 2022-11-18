use crate::prelude::*;

////////////////////////////////////////////////////////////
// Point2d
////////////////////////////////////////////////////////////

/// A trait for types representing a 2d Point.
pub trait Point2d: Clone + Copy {
    #[allow(clippy::new_ret_no_self)]
    /// Construct a IVec2
    fn new(x: i32, y: i32) -> IVec2 {
        IVec2::new(x, y)
    }

    /// Returns x position.
    fn x(&self) -> i32;

    /// Returns y position.
    fn y(&self) -> i32;

    /// Convert point to `IVec2` (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    /// Convert point to `UVec2` (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 {
        self.as_ivec2().as_uvec2()
    }

    /// Convert point to `Vec2` (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    /// Convert point to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }

    /// Get the point's corresponding 1d index.
    #[inline(always)]
    fn as_index(&self, width: usize) -> usize {
        self.y() as usize * width + self.x() as usize
    }

    /// Returns true if the point is valid for the given size.
    #[inline]
    fn is_valid(&self, size: impl Size2d) -> bool {
        let x = self.x();
        let y = self.y();

        x >= 0 && y >= 0 && (x as u32) < size.width() && (y as u32) < size.height()
    }

    ////////////////
    //  Geometry  //
    ////////////////
    #[inline]
    fn from_angle(center: impl Point2d, distance: f32, degrees: f32) -> IVec2 {
        let rads = degrees.to_radians();
        let x = (distance * rads.cos()).floor() as i32; // .round() ??
        let y = (distance * rads.sin()).floor() as i32;

        IVec2::new(center.x() + x, center.y() + y)
    }

    #[inline]
    fn angle_to(&self, point: impl Point2d) -> f32 {
        let x = (point.x() - self.x()) as f32;
        let y = (point.y() - self.y()) as f32;
        y.atan2(x).to_degrees()
    }

    #[inline]
    fn mid_point(&self, point: impl Point2d) -> IVec2 {
        IVec2 { x: (self.x() + point.x()) / 2, y: (self.y() + point.y()) / 2 }
    }

    #[inline]
    fn cross_product(&self, point: impl Point2d) -> i32 {
        self.x() * point.y() - self.y() * point.x()
    }

    #[inline]
    fn dot_product(&self, point: impl Point2d) -> i32 {
        self.x() * point.x() + self.y() * point.y()
    }
}

#[macro_export]
macro_rules! impl_grid_point_array {
    ($type:ty) => {
        impl Point2d for $type {
            fn x(&self) -> i32 {
                self[0] as i32
            }

            fn y(&self) -> i32 {
                self[1] as i32
            }
        }
    };
}

#[macro_export]
macro_rules! impl_grid_point_tuple {
    ($type:ty) => {
        impl Point2d for $type {
            fn x(&self) -> i32 {
                self.0 as i32
            }

            fn y(&self) -> i32 {
                self.1 as i32
            }
        }
    };
}

impl_grid_point_array!(IVec2);
impl_grid_point_array!(UVec2);
impl_grid_point_tuple!((u32, u32));
impl_grid_point_tuple!((i32, i32));
impl_grid_point_tuple!((usize, usize));

impl Point2d for Vec2 {
    fn x(&self) -> i32 {
        self.x.floor() as i32
    }
    fn y(&self) -> i32 {
        self.y.floor() as i32
    }
}

impl Point2d for (f32, f32) {
    fn x(&self) -> i32 {
        self.0.floor() as i32
    }
    fn y(&self) -> i32 {
        self.1.floor() as i32
    }
}
