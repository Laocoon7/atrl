use crate::prelude::*;

#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct Equipable {
    pub location: EquipmentSlot,
    #[reflect(ignore)]
    pub available_locations: Vec<EquipmentSlot>,
}
