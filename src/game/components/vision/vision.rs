use crate::game::components::*;
use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Debug)]
pub struct Vision {
    pub vision_types: Vec<VisionType>,
}
