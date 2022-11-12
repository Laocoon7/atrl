use crate::game::components::*;
use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component, Debug)]
pub struct Movement {
    pub movement_types: Vec<MovementType>,
}
