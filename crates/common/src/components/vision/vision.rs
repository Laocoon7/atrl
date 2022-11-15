use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default)]
#[reflect(Component)]
pub struct Vision {
    pub vision_types: u8, // This must match with vision_type
}
