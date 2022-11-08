use crate::game::prelude::*;

#[derive(Inspectable, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum MovementType {
    None = 0,
    Walk,
    Run,
    Swim,
    Fly,
    Phase,
}

impl Default for MovementType {
    fn default() -> Self { MovementType::None }
}
