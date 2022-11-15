use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default)]
#[reflect(Component)]
pub struct Movement {
    pub movement_types: Vec<MovementType>,
}
