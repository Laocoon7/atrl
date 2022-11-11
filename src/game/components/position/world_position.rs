use atrl_engine::bevy_math::Vec3Swizzles;

use crate::game::prelude::*;

#[derive(
    Inspectable, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
pub struct WorldPosition {
    pub position: IVec3,
}

impl WorldPosition {
    pub fn xy(&self) -> IVec2 { self.position.xy() }
}
