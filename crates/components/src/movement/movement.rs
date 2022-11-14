use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Movement {
    pub movement_types: Vec<MovementType>,
}
