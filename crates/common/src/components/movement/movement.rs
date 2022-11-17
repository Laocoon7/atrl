use crate::prelude::*;

#[derive(Reflect, Component, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Movement(pub u8); // this must match with movement_type
