use crate::game::prelude::*;

#[derive(Inspectable, Component)]
pub struct Equipable {
    pub location: EquipmentSlot,
    pub available_locations: Vec<EquipmentSlot>,
}
