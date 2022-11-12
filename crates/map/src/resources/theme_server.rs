pub use crate::prelude::*;

pub struct ThemeServer {
    pub tilesets: HashMap<String, (Handle<TextureAtlas>, Tileset)>,
    pub themes: HashMap<String, Theme>,
}

impl ThemeServer {
    pub fn get_theme(&self, theme_name: &str) -> Option<&Theme> { self.themes.get(theme_name) }

    pub fn get_tileset(&self, tileset_name: &str) -> Option<&(Handle<TextureAtlas>, Tileset)> {
        self.tilesets.get(tileset_name)
    }
}
