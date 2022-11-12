use crate::prelude::*;

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
            Self::None => vec![],
            Self::Wall => vec![MovementType::Fly, MovementType::Phase],
            Self::Floor => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            Self::Water => vec![MovementType::Swim, MovementType::Fly, MovementType::Phase],
        }
    }

    /// The tile is visible to these vision types (but not necessarily explored)
    pub fn allowed_vision(&self) -> Vec<VisionType> {
        match self {
            Self::None => vec![],
            Self::Wall => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::Floor => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::Water => vec![
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
            Self::None => vec![],
            Self::Wall => vec![VisionType::XRay],
            Self::Floor => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::Water => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
        }
    }
}

impl From<TerrainType> for u8 {
    fn from(value: TerrainType) -> Self {
        ToPrimitive::to_u8(&value).expect("Failed to convert `TerrainType` to u8")
    }
}

impl From<TerrainType> for u64 {
    fn from(value: TerrainType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `TerrainType` to u64")
    }
}
