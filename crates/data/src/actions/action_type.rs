use crate::prelude::*;

#[derive(Reflect, FromReflect)]
pub enum ActionType {
    Wait,
    Movement(IVec2),
}
