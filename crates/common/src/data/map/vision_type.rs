use bevy::reflect::FromReflect;

use crate::prelude::{Deserialize, Reflect, Serialize};

#[derive(
    Reflect, FromReflect, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug,
)]
pub enum VisionType {
    #[default]
    Blind = 0,
    BlackAndWhite,
    Colored,
    Infared,
    XRay,
}
