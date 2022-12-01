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
pub struct LocalPosition(pub UVec2);
