use crate::prelude::*;

#[derive(Deserialize)]
pub struct TileTemplate {
    pub x: usize,
    pub y: usize,
    pub tile_type: u8,
    pub foreground_color: Option<ColorDefinition>,
    pub background_color: Option<ColorDefinition>,
}
