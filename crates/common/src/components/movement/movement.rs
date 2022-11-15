use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default)]
#[reflect(Component)]
pub struct Movement {
    pub movement_types: u8, // this must match with movement_type
}
