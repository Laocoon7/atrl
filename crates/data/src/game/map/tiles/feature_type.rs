use crate::prelude::*;

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
    pub fn allowed_movement(&self) -> u8 {
        match self {
            Self::None => MovementType::Any as u8,
            Self::StairsDown => {
                (MovementType::Walk as u8)
                    | (MovementType::Run as u8)
                    | (MovementType::Fly as u8)
                    | (MovementType::Phase as u8)
            }
            Self::StairsUp => {
                (MovementType::Walk as u8)
                    | (MovementType::Run as u8)
                    | (MovementType::Fly as u8)
                    | (MovementType::Phase as u8)
            }
            Self::DoorClosed => {
                (MovementType::Walk as u8)
                    | (MovementType::Run as u8)
                    | (MovementType::Fly as u8)
                    | (MovementType::Phase as u8)
            }
            Self::DoorOpen => {
                (MovementType::Walk as u8)
                    | (MovementType::Run as u8)
                    | (MovementType::Fly as u8)
                    | (MovementType::Phase as u8)
            }
        }
    }

    /// The tile is visible to these vision types (but not necessarily explored)
    pub fn allowed_vision(&self) -> u8 {
        match self {
            Self::None => VisionType::None as u8,
            Self::StairsDown => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::StairsUp => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::DoorClosed => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::DoorOpen => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
        }
    }

    /// The tile is considered opaque unless VisionComponent includes one of these types
    pub fn vision_penetrates(&self) -> u8 {
        match self {
            Self::None => VisionType::None as u8,
            Self::StairsDown => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::StairsUp => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
            Self::DoorClosed => VisionType::XRay as u8,
            Self::DoorOpen => {
                (VisionType::BlackAndWhite as u8)
                    | (VisionType::Colored as u8)
                    | (VisionType::Infared as u8)
                    | (VisionType::XRay as u8)
            }
        }
    }
}

impl From<FeatureType> for u32 {
    fn from(value: FeatureType) -> Self {
        ToPrimitive::to_u32(&value).expect("Failed to convert `FeatureType` to u32")
    }
}

impl From<FeatureType> for u64 {
    fn from(value: FeatureType) -> Self {
        ToPrimitive::to_u64(&value).expect("Failed to convert `FeatureType` to u64")
    }
}
