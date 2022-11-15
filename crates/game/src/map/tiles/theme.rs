use crate::prelude::*;

pub const DEFAULT_TERRAIN_THEME_NAME: &str = "terminal8x8";
pub const DEFAULT_FEATURE_THEME_NAME: &str = "terminal8x8";
pub const DEFAULT_ITEM_THEME_NAME: &str = "terminal8x8";

pub struct Theme {
    pub tiles: HashMap<u8, TileDefinition>,
}

impl Theme {
    pub fn get_tile_definition(&self, tile_type: u8) -> Option<&TileDefinition> {
        if self.tiles.contains_key(&tile_type) {
            Some(&self.tiles[&tile_type])
        } else {
            None
        }
    }
}
