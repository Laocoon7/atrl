use crate::prelude::*;
#[derive(
    Reflect,
    FromReflect,
    Default,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Debug,
    FromPrimitive,
    ToPrimitive,
)]
pub enum MovementType {
    #[default]
    None = 0,
    Walk = 1 << 0,
    Run = 1 << 1,
    Swim = 1 << 2,
    Fly = 1 << 3,
    Phase = 1 << 4,
    Any = !0,
}

impl TryFrom<MovementType> for u8 {
    type Error = String;

    fn try_from(value: MovementType) -> Result<Self, Self::Error> {
        value.to_u8().map_or(
            Err("Failed to convert `MovementType` to `u8`".to_string()),
            Ok,
        )
    }
}

impl MovementType {
    pub fn as_u8(self) -> u8 { self.try_into().unwrap_or(Self::None as u8) }
}

impl_as_primative!(MovementType);
