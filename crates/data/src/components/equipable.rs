use crate::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Equipable {
    pub location: EquipmentSlot,
    pub available_locations: Vec<EquipmentSlot>,
}
