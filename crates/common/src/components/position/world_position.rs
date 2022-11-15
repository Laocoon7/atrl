use crate::prelude::*;
use bevy::math::Vec3Swizzles;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct WorldPosition {
    pub position: IVec3,
}

impl WorldPosition {
    pub fn xy(&self) -> IVec2 { self.position.xy() }
}
