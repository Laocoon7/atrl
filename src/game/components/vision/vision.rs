use crate::game::components::*;
use crate::prelude::*;

#[derive(Inspectable, Component, Debug)]
pub struct Vision {
    pub vision_types: Vec<VisionType>,
}
