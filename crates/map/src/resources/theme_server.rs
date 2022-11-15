pub use crate::prelude::*;

#[derive(Resource)]
pub struct ThemeServer {
    pub themes: HashMap<String, Theme>,
    pub tilesets: HashMap<String, (Handle<TextureAtlas>, Tileset)>,
}

impl ThemeServer {
    pub fn get_theme(&self, theme_name: &str) -> Option<&Theme> { self.themes.get(theme_name) }

    pub fn get_tileset(&self, tileset_name: &str) -> Option<&(Handle<TextureAtlas>, Tileset)> {
        self.tilesets.get(tileset_name)
    }
}

impl Default for ThemeServer {
    fn default() -> Self { Self { tilesets: HashMap::new(), themes: HashMap::new() } }
}