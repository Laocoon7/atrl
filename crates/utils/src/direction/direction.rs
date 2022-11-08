use super::*;
use crate::{prelude::*, Point2d};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};

pub const NUM_DIRECTIONS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum GridDirection {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DirectionType {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection),
}

impl GridDirection {
    pub fn from_unit_coord<P>(coord: impl Point2d + std::fmt::Debug) -> Self {
        match [coord.x(), coord.y()] {
            [1, 0] => GridDirection::East,
            [-1, 0] => GridDirection::West,
            [0, 1] => GridDirection::South,
            [0, -1] => GridDirection::North,
            [1, 1] => GridDirection::SouthEast,
            [1, -1] => GridDirection::NorthEast,
            [-1, 1] => GridDirection::SouthWest,
            [-1, -1] => GridDirection::NorthWest,
            _ => panic!("Unexpected coord: {:?}", coord),
        }
    }

    pub fn opposite(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::South,
            GridDirection::NorthEast => GridDirection::SouthWest,
            GridDirection::East => GridDirection::West,
            GridDirection::SouthEast => GridDirection::NorthWest,
            GridDirection::South => GridDirection::North,
            GridDirection::SouthWest => GridDirection::NorthEast,
            GridDirection::West => GridDirection::East,
            GridDirection::NorthWest => GridDirection::SouthEast,
        }
    }

    pub fn coord(self) -> IVec2 {
        match self {
            GridDirection::North => IVec2::new(0, -1),
            GridDirection::NorthEast => IVec2::new(1, -1),
            GridDirection::East => IVec2::new(1, 0),
            GridDirection::SouthEast => IVec2::new(1, 1),
            GridDirection::South => IVec2::new(0, 1),
            GridDirection::SouthWest => IVec2::new(-1, 1),
            GridDirection::West => IVec2::new(-1, 0),
            GridDirection::NorthWest => IVec2::new(-1, -1),
        }
    }

    pub fn left90(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::West,
            GridDirection::NorthEast => GridDirection::NorthWest,
            GridDirection::East => GridDirection::North,
            GridDirection::SouthEast => GridDirection::NorthEast,
            GridDirection::South => GridDirection::East,
            GridDirection::SouthWest => GridDirection::SouthEast,
            GridDirection::West => GridDirection::South,
            GridDirection::NorthWest => GridDirection::SouthWest,
        }
    }

    pub fn right90(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::East,
            GridDirection::NorthEast => GridDirection::SouthEast,
            GridDirection::East => GridDirection::South,
            GridDirection::SouthEast => GridDirection::SouthWest,
            GridDirection::South => GridDirection::West,
            GridDirection::SouthWest => GridDirection::NorthWest,
            GridDirection::West => GridDirection::North,
            GridDirection::NorthWest => GridDirection::NorthEast,
        }
    }

    pub fn left45(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::NorthWest,
            GridDirection::NorthEast => GridDirection::North,
            GridDirection::East => GridDirection::NorthEast,
            GridDirection::SouthEast => GridDirection::East,
            GridDirection::South => GridDirection::SouthEast,
            GridDirection::SouthWest => GridDirection::South,
            GridDirection::West => GridDirection::SouthWest,
            GridDirection::NorthWest => GridDirection::West,
        }
    }

    pub fn right45(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::NorthEast,
            GridDirection::NorthEast => GridDirection::East,
            GridDirection::East => GridDirection::SouthEast,
            GridDirection::SouthEast => GridDirection::South,
            GridDirection::South => GridDirection::SouthWest,
            GridDirection::SouthWest => GridDirection::West,
            GridDirection::West => GridDirection::NorthWest,
            GridDirection::NorthWest => GridDirection::North,
        }
    }

    pub fn left135(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::SouthWest,
            GridDirection::NorthEast => GridDirection::West,
            GridDirection::East => GridDirection::NorthWest,
            GridDirection::SouthEast => GridDirection::North,
            GridDirection::South => GridDirection::NorthEast,
            GridDirection::SouthWest => GridDirection::East,
            GridDirection::West => GridDirection::SouthEast,
            GridDirection::NorthWest => GridDirection::South,
        }
    }

    pub fn right135(self) -> GridDirection {
        match self {
            GridDirection::North => GridDirection::SouthEast,
            GridDirection::NorthEast => GridDirection::South,
            GridDirection::East => GridDirection::SouthWest,
            GridDirection::SouthEast => GridDirection::West,
            GridDirection::South => GridDirection::NorthWest,
            GridDirection::SouthWest => GridDirection::North,
            GridDirection::West => GridDirection::NorthEast,
            GridDirection::NorthWest => GridDirection::East,
        }
    }

    pub const fn bitmap_raw(self) -> u8 { 1 << self as usize }

    pub const fn bitmap(self) -> DirectionBitmap { DirectionBitmap::new(self.bitmap_raw()) }

    pub fn is_cardinal(self) -> bool {
        matches!(
            self,
            GridDirection::North | GridDirection::East | GridDirection::South | GridDirection::West
        )
    }

    pub fn is_ordinal(self) -> bool {
        matches!(
            self,
            GridDirection::NorthEast
                | GridDirection::SouthEast
                | GridDirection::SouthWest
                | GridDirection::NorthWest
        )
    }

    pub fn typ(self) -> DirectionType {
        match self {
            GridDirection::North => DirectionType::Cardinal(CardinalDirection::North),
            GridDirection::NorthEast => DirectionType::Ordinal(OrdinalDirection::NorthEast),
            GridDirection::East => DirectionType::Cardinal(CardinalDirection::East),
            GridDirection::SouthEast => DirectionType::Ordinal(OrdinalDirection::SouthEast),
            GridDirection::South => DirectionType::Cardinal(CardinalDirection::South),
            GridDirection::SouthWest => DirectionType::Ordinal(OrdinalDirection::SouthWest),
            GridDirection::West => DirectionType::Cardinal(CardinalDirection::West),
            GridDirection::NorthWest => DirectionType::Ordinal(OrdinalDirection::NorthWest),
        }
    }

    pub fn cardinal(self) -> Option<CardinalDirection> {
        match self {
            GridDirection::North => Some(CardinalDirection::North),
            GridDirection::East => Some(CardinalDirection::East),
            GridDirection::South => Some(CardinalDirection::South),
            GridDirection::West => Some(CardinalDirection::West),
            _ => None,
        }
    }

    pub fn ordinal(self) -> Option<OrdinalDirection> {
        match self {
            GridDirection::NorthEast => Some(OrdinalDirection::NorthEast),
            GridDirection::SouthEast => Some(OrdinalDirection::SouthEast),
            GridDirection::SouthWest => Some(OrdinalDirection::SouthWest),
            GridDirection::NorthWest => Some(OrdinalDirection::NorthWest),
            _ => None,
        }
    }

    pub const fn all() -> DirectionIter { DirectionIter::new() }
}

impl From<GridDirection> for [i32; 2] {
    fn from(d: GridDirection) -> [i32; 2] {
        use self::GridDirection::*;
        match d {
            North => [0, -1],
            East => [1, 0],
            South => [0, 1],
            West => [-1, 0],
            NorthWest => [-1, -1],
            NorthEast => [1, -1],
            SouthEast => [1, 1],
            SouthWest => [-1, 1],
        }
    }
}
impl From<GridDirection> for (i32, i32) {
    fn from(d: GridDirection) -> (i32, i32) {
        use self::GridDirection::*;
        match d {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 1),
            SouthWest => (-1, 1),
        }
    }
}

impl Distribution<GridDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridDirection {
        let index = rng.gen_range(0..NUM_DIRECTIONS as u8);
        unsafe { std::mem::transmute(index) }
    }
}
