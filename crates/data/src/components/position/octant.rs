use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Octant(pub u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    /// converts a `Position` into a coordinate relative `Octant(0)` offset

    #[inline]
    pub fn to_offset(&self, position: Position) -> (i64, i64) {
        let offset = position.absolute_position();
        match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (offset.1, -offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (-offset.1, offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        }
    }

    /// converts from a `Octant(0)` relative coordinate into a `Position`
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_offset(&self, offset: (i64, i64), z: i32, layer: u32) -> Position {
        let p = match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (-offset.1, offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (offset.1, -offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        };
        Position::from_absolute_position((p.0, p.1, z), layer)
    }
}
