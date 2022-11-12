use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Health {
    current_hp: i32,
    max_hp: i32,
}

impl_new!(Health, current_hp: i32, max_hp: i32);
