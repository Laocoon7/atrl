use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub tileset_name: String,

    pub tile_type: u16,

    pub xy: Option<(usize, usize)>,
    pub index: Option<usize>,

    pub foreground_color: Option<ColorDefinition>,
    pub background_color: Option<ColorDefinition>,
}
