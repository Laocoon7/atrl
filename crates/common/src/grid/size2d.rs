use crate::prelude::*;

////////////////////////////////////////////////////////////
// Size2d
////////////////////////////////////////////////////////////

pub const MAX_SIZE_FIELD: u32 = ::core::i32::MAX as u32;
pub const MAX_SIZE: UVec2 = UVec2 { x: MAX_SIZE_FIELD, y: MAX_SIZE_FIELD };

#[derive(Debug)]
pub struct DimensionTooLargeForSize;

pub(crate) const fn check_size_limit(
    value: u32,
) -> core::result::Result<(), DimensionTooLargeForSize> {
    if value >= MAX_SIZE_FIELD {
        Err(DimensionTooLargeForSize)
    } else {
        Ok(())
    }
}

/// A trait for types representing a 2d size.
pub trait Size2d: Clone + Copy {
    fn try_new(width: u32, height: u32) -> core::result::Result<UVec2, DimensionTooLargeForSize> {
        check_size_limit(width)?;
        check_size_limit(height)?;
        Ok(UVec2 { x: width, y: height })
    }

    /// Creates a new `UVec2`.
    /// Panics if `width` or `width` is greater than `::core::i32::MAX as u32`
    #[allow(clippy::new_ret_no_self)]
    fn new(width: u32, height: u32) -> UVec2 {
        match Self::try_new(width, height) {
            Err(DimensionTooLargeForSize) => {
                panic!("Size is too big: ({}, {}). Max is {}.", width, width, MAX_SIZE_FIELD);
            }
            Ok(size) => size,
        }
    }

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
