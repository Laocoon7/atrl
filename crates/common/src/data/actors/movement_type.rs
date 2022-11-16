use crate::prelude::*;

#[derive(
    Reflect, FromReflect, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug,
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
    fn from(value: MovementType) -> Self { value as u8 }
}
