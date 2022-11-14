use crate::prelude::*;

#[derive(Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
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
