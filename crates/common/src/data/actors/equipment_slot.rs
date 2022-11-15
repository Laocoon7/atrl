use bevy::reflect::Reflect;

#[derive(Reflect, Default, Debug)]
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
