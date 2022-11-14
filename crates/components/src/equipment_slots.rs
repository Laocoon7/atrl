use crate::prelude::*;

#[derive(Default, Debug, Reflect)]
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
