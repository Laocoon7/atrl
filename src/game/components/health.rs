use crate::game::prelude::*;

#[derive(Inspectable, Component)]
pub struct Health {
    current_hp: i32,
    max_hp: i32,
}

impl_new!(Health, current_hp: i32, max_hp: i32);
