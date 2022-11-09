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
    DownStairs,
    UpStairs,
}

impl FeatureType {
    /// Movement is allowed if MovementComponent allows any of these types
    pub fn allowed_movement(&self) -> Vec<MovementType> {
        match self {
            FeatureType::None => vec![],
            FeatureType::DownStairs => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
            FeatureType::UpStairs => {
                vec![MovementType::Walk, MovementType::Run, MovementType::Fly, MovementType::Phase]
            }
        }
    }

    /// The tile is visible to these vision types (but not necessarily explored)
    pub fn allowed_vision(&self) -> Vec<VisionType> {
        match self {
            FeatureType::None => vec![],
            FeatureType::DownStairs => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            FeatureType::UpStairs => vec![
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
            FeatureType::None => vec![],
            FeatureType::DownStairs => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
            FeatureType::UpStairs => vec![
                VisionType::BlackAndWhite,
                VisionType::Colored,
                VisionType::Infared,
                VisionType::XRay,
            ],
        }
    }

    pub const fn is_walkable(self) -> bool { matches!(self, Self::DownStairs) }
}

impl From<FeatureType> for u64 {
    fn from(value: FeatureType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `FeatureType` to u64")
    }
}

impl From<FeatureType> for usize {
    fn from(value: FeatureType) -> Self {
        match value {
            FeatureType::None => 0,
            FeatureType::DownStairs => from_cp437('>'),
            FeatureType::UpStairs => from_cp437('<'),
        }
    }
}

impl From<FeatureType> for Color {
    fn from(value: FeatureType) -> Self {
        match value {
            FeatureType::None => Color::NONE,
            FeatureType::DownStairs => Color::WHITE,
            FeatureType::UpStairs => Color::WHITE,
        }
    }
}
