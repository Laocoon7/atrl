use crate::prelude::*;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(
    Reflect,
    FromReflect,
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
#[repr(u32)]
pub enum TerrainType {
    #[default]
    None,
    Floor,
    Wall,
    Water,
}

impl TerrainType {
    /// Movement is allowed if MovementComponent allows any of these types
    pub fn allowed_movement(&self) -> u8 {
        match self {
            Self::None => (MovementType::None) as u8,
            Self::Wall => (MovementType::Fly as u8) | (MovementType::Phase as u8),
            Self::Floor => {
                (MovementType::Walk as u8)
                    | (MovementType::Run as u8)
                    | (MovementType::Fly as u8)
                    | (MovementType::Phase as u8)
            }
            Self::Water => {
                (MovementType::Swim as u8) | (MovementType::Fly as u8) | (MovementType::Phase as u8)
            }
        }
    }

    /// The tile is opaque except to these vision types
    pub fn vision_penetrates(&self) -> u8 {
        match self {
            Self::None => VisionType::None as u8,
            Self::Wall => VisionType::XRay as u8,
            Self::Floor => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::Water => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
        }
    }
}

impl From<TerrainType> for u32 {
    fn from(value: TerrainType) -> Self {
        ToPrimitive::to_u32(&value).expect("Failed to convert `TerrainType` to u32")
    }
}
