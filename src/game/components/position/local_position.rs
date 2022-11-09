use crate::game::prelude::*;

#[derive(
    Inspectable, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
pub struct LocalPosition {
    position: UVec2,
}
