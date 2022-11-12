use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MovementType {
    None = 0,
    Walk,
    Run,
    Swim,
    Fly,
    Phase,
}

impl Default for MovementType {
    fn default() -> Self { Self::None }
}
