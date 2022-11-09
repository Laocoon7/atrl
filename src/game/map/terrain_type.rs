use crate::game::prelude::*;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(
    Debug,
    Default,
    FromPrimitive,
    ToPrimitive,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum TerrainType {
    #[default]
    None,
    Wall,
    Floor,
    Water,
}

impl TerrainType {
    /// Movement is allowed if MovementComponent allows any of these types
    pub fn allowed_movement(&self) -> Vec<MovementType> {
        match self {
            TerrainType::None => vec![],
            TerrainType::Wall => vec![MovementType::Fly, MovementType::Phase],
            TerrainType::Floor => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            TerrainType::Water => vec![MovementType::Swim, MovementType::Fly, MovementType::Phase],
        }
    }

    /// The tile is visible to these vision types (but not necessarily explored)
    pub fn allowed_vision(&self) -> Vec<VisionType> {
        match self {
            TerrainType::None => vec![],
            TerrainType::Wall => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            TerrainType::Floor => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            TerrainType::Water => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
        }
    }

    /// The tile is considered opaque unless VisionComponent includes one of these types
    pub fn vision_penetrates(&self) -> Vec<VisionType> {
        match self {
            TerrainType::None => vec![],
            TerrainType::Wall => vec![VisionType::XRay],
            TerrainType::Floor => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            TerrainType::Water => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
        }
    }
}

impl From<TerrainType> for u64 {
    fn from(value: TerrainType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `TerrainType` to u64")
    }
}

impl From<TerrainType> for usize {
    fn from(value: TerrainType) -> Self {
        match value {
            TerrainType::None => 0,
            TerrainType::Wall => from_cp437('#'),
            TerrainType::Floor => from_cp437('.'),
            TerrainType::Water => from_cp437('~'),
        }
    }
}

impl From<TerrainType> for Color {
    fn from(value: TerrainType) -> Self {
        match value {
            TerrainType::None => Color::NONE,
            TerrainType::Wall => Color::GREEN,
            TerrainType::Floor => Color::GRAY,
            TerrainType::Water => Color::BLUE,
        }
    }
}
