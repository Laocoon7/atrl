use crate::prelude::*;

#[derive(
    Reflect, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
#[reflect(Component)]
pub struct LocalPosition {
    pub position: IVec2,
}

impl_new!(LocalPosition, position: IVec2);

impl AtrlPosition2 for LocalPosition {
    type Position = IVec2;

    fn get(&self) -> Self::Position { self.position }

    fn set_value(&mut self, pos: Self::Position) { self.position = pos }
}
