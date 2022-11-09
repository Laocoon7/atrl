use crate::game::prelude::*;

#[derive(Inspectable, Default, Debug)]
pub enum EquipmentSlot {
    #[default]
    Head,
    Chest,
    Shoulders,
    Back,
    Arms,
    Hands,
    Waist,
    Legs,
    Feet,
}
