use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GridAxis {
    X,
    Y,
}

impl GridAxis {
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            GridAxis::X => GridAxis::Y,
            GridAxis::Y => GridAxis::X,
        }
    }

    #[inline(always)]
    pub fn new_coord<P: Point2d>(self, this_axis: i32, other_axis: i32) -> IVec2 {
        match self {
            GridAxis::X => P::new(this_axis, other_axis),
            GridAxis::Y => P::new(other_axis, this_axis),
        }
    }

    pub fn try_new_size(self, this_axis: u32, other_axis: u32) -> Option<UVec2> {
        match self {
            GridAxis::X => UVec2::new_try(this_axis, other_axis),
            GridAxis::Y => UVec2::new_try(other_axis, this_axis),
        }
    }

    #[inline]
    pub fn new_size(self, this_axis: u32, other_axis: u32) -> UVec2 {
        match self {
            GridAxis::X => UVec2::new(this_axis, other_axis),
            GridAxis::Y => UVec2::new(other_axis, this_axis),
        }
    }

    #[inline]
    pub fn size<S: Size2d>(self, size: S) -> u32 {
        match self {
            GridAxis::X => size.width(),
            GridAxis::Y => size.height(),
        }
    }
}
