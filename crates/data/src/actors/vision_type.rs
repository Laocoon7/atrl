use crate::prelude::*;
use atrl_macros::impl_as_primative;

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
#[repr(u8)] // this must match with vision component
pub enum VisionType {
    #[default]
    None = 0,
    Blind = 1 << 0,
    BlackAndWhite = 1 << 1,
    Colored = 1 << 2,
    Infared = 1 << 3,
    XRay = 1 << 4,
    Any = !0,
}

impl TryFrom<VisionType> for u8 {
    type Error = String;

    fn try_from(value: VisionType) -> Result<Self, Self::Error> {
        value.to_u8().map_or(Err("Failed to convert `VisionType` to `u8`".to_string()), Ok)
    }
}

impl VisionType {
    pub fn as_u8(self) -> u8 {
        self.try_into().unwrap()
    }
}

impl_as_primative!(VisionType);
