use crate::prelude::*;

#[derive(
    Reflect,
    Component,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Deref,
    DerefMut,
)]
#[reflect(Component)]
pub struct LocalPosition(pub IVec2,);

// TODO: Are we even using this???

impl AtrlPosition2 for LocalPosition {
    type Position = IVec2;

    fn get(&self,) -> Self::Position { **self }

    fn set_value(&mut self, pos: Self::Position,) { **self = pos }
}
