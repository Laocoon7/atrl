use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Component)]
pub struct Equipable {
    pub location: EquipmentSlot,
    pub available_locations: Vec<EquipmentSlot>,
}
