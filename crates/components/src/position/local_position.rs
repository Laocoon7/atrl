use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LocalPosition {
    pub position: IVec2,
}

impl_new!(LocalPosition, position: IVec2);
