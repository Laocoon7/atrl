use bevy::reflect::FromReflect;

use crate::prelude::{Deserialize, Reflect, Serialize};

#[derive(
    Reflect, FromReflect, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug,
)]
pub enum MovementType {
    #[default]
    None = 0,
    Walk,
    Run,
    Swim,
    Fly,
    Phase,
}
