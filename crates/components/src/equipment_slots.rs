#[cfg(feature = "debug")]
use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Default, Debug)]
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
