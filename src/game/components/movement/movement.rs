use crate::game::components::*;
use crate::prelude::*;

#[derive(Inspectable, Component, Debug)]
pub struct Movement {
    pub movement_types: Vec<MovementType>,
}
