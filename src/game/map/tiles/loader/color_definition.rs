use crate::game::prelude::*;

#[derive(Deserialize)]
pub struct ColorDefinition {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
