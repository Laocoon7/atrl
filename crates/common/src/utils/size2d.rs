use crate::prelude::*;

////////////////////////////////////////////////////////////
// Size2d
////////////////////////////////////////////////////////////

const MAX_SIZE: u32 = i32::MAX as u32;

#[derive(Debug)]
pub struct DimensionTooLargeForSize;

const fn check_size(value: u32) -> bool {
    value <= MAX_SIZE
}

/// A trait for types representing a 2d size.
#[allow(clippy::new_ret_no_self)]
pub trait Size2d: Clone + Copy {
    // Safely create a new UVec2
    fn new_try(width: u32, height: u32) -> Option<UVec2> {
        if check_size(width) && check_size(height) {
            Some(UVec2::new(width, height))
        } else {
            None
        }
    }

    // Create a new UVec2
    // Panics if `width` or `height` is greater than `i32::MAX`
    fn new(width: u32, height: u32) -> UVec2 {
        match Self::new_try(width, height) {
            Some(size) => size,
            None => panic!("Size is too big: ({}, {}). Max is {}.", width, height, MAX_SIZE),
        }
    }

    /// Returns width value.
    fn width(&self) -> u32;

    /// Returns height value.
    fn height(&self) -> u32;

    #[inline]
    fn count(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    /// Convert dimensions to UVec2 (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    /// Convert dimensions to IVec2 (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 {
        self.as_uvec2().as_ivec2()
    }

    /// Convert dimensions to `Vec2` (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 {
        self.as_uvec2().as_vec2()
    }

    /// Convert dimensions to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [u32; 2] {
        self.as_uvec2().to_array()
    }

    /// Returns true if the point is valid within the size.
    #[inline]
    fn contains(&self, point: impl Point2d) -> bool {
        point.x() >= 0
            && point.y() >= 0
            && (point.x() as u32) < self.width()
            && (point.y() as u32) < self.height()
    }

    /// Returns an iterator over all points within the size.
    fn iter(self) -> PointIterRowMajor {
        PointIterRowMajor::new(self)
    }
}

#[macro_export]
macro_rules! impl_size2d_array {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 {
                self[0] as u32
            }

            fn height(&self) -> u32 {
                self[1] as u32
            }
        }
    };
}

#[macro_export]
macro_rules! impl_size2d_tuple {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> u32 {
                self.0 as u32
            }

            fn height(&self) -> u32 {
                self.1 as u32
            }
        }
    };
}

impl_size2d_array!(IVec2);
impl_size2d_array!(UVec2);
impl_size2d_array!([u32; 2]);
impl_size2d_array!([i32; 2]);
impl_size2d_array!([usize; 2]);

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
