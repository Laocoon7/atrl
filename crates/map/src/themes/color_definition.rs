use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ColorDefinition {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<&ColorDefinition> for Color {
    fn from(value: &ColorDefinition) -> Self { Color::rgba_u8(value.r, value.g, value.b, value.a) }
}

//impl_serialized_object_for!(ColorDefinition);
