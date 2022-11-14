use crate::prelude::*;

#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct Equipable {
    pub location: EquipmentSlot,
    // pub available_locations: Vec<EquipmentSlot>,
}
