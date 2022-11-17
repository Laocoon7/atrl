use crate::prelude::*;
use atrl_macros::impl_as_primative;

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

impl From<MovementType> for u8 {
    fn from(value: MovementType) -> Self {
        value.to_u8().unwrap()
    }
}

impl MovementType {
    pub fn as_u8(self) -> u8 {
        self.into()
    }
}

impl_as_primative!(MovementType);
