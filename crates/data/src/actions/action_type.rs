use crate::prelude::*;

#[derive(Debug, Reflect, FromReflect)]
pub enum ActionType {
    Wait,
    Movement(IVec2),
}
