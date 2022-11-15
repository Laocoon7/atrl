use bevy::reflect::Reflect;

#[derive(Reflect)]
pub enum AIType {
    Human,
    Aggressive,
    Scared,
}
