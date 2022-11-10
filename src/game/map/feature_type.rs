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
pub enum FeatureType {
    #[default]
    None,
    StairsDown,
    StairsUp,
    DoorClosed,
    DoorOpen,
}

impl FeatureType {
    /// Movement is allowed if MovementComponent allows any of these types
    pub fn allowed_movement(&self) -> Vec<MovementType> {
        match self {
            Self::None => vec![],
            Self::StairsDown => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            Self::StairsUp => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            Self::DoorClosed => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            Self::DoorOpen => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
        }
    }

    /// The tile is visible to these vision types (but not necessarily explored)
    pub fn allowed_vision(&self) -> Vec<VisionType> {
        match self {
            Self::None => vec![],
            Self::StairsDown => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::StairsUp => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::DoorClosed => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::DoorOpen => vec![
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
            Self::StairsDown => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::StairsUp => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            Self::DoorClosed => vec![VisionType::XRay],
            Self::DoorOpen => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
        }
    }
}

impl From<FeatureType> for u8 {
    fn from(value: FeatureType) -> Self {
        ToPrimitive::to_u8(&value).expect("Failed to convert `TerrainType` to u8")
    }
}

impl From<FeatureType> for u64 {
    fn from(value: FeatureType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `FeatureType` to u64")
    }
}
