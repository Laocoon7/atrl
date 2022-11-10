use crate::game::prelude::*;

#[derive(
    Inspectable, Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug,
)]
pub struct WorldPosition {
    pub position: IVec3,
}

impl WorldPosition {
    pub const fn xy(&self) -> IVec2 { IVec2::new(self.position.x, self.position.y) }
}
