use crate::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Health {
    current_hp: i32,
    max_hp: i32,
}

impl_new!(Health, current_hp: i32, max_hp: i32);
