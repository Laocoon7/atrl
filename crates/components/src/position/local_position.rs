use crate::prelude::*;

#[derive(
    Reflect, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
#[reflect(Component)]
pub struct LocalPosition {
    pub position: IVec2,
}

impl_new!(LocalPosition, position: IVec2);
