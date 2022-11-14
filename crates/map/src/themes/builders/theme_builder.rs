use crate::prelude::*;

#[derive(Default)]
pub struct ThemeBuilder {
    pub name: Option<String>,
    pub theme_type: Option<u8>,

    pub tiles: Vec<Tile>,
    pub animations: Vec<Animation>,
}

impl ThemeBuilder {
    /// Creates a new `ThemeBuilder`
    /// The following `.set_XXX` **must** be called prior to `.build()`:
    /// `.set_name()`
    /// `.set_theme_type()`
    pub fn new() -> Self {
        Self { name: None, theme_type: None, tiles: Vec::new(), animations: Vec::new() }
    }

    #[must_use]
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    #[must_use]
    pub fn set_theme_type<T: Into<u8>>(mut self, theme_type: T) -> Self {
        self.theme_type = Some(theme_type.into());
        self
    }

    pub fn add_tile(mut self, tile: Tile) -> Self {
        self.tiles.push(tile);
        self
    }

    pub fn add_animation(mut self, animation: Animation) -> Self {
        self.animations.push(animation);
        self
    }

    pub fn build(self) -> Result<Theme> {
        let name = match self.name {
            Some(u) => u,
            None => return Err(MyError::BuilderError("Theme".to_string(), "name".to_string())),
        };

        let theme_type = match self.theme_type {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Theme".to_string(), "theme_type".to_string()))
            }
        };

        let tiles = self.tiles;
        let animations = self.animations;

        Ok(Theme { name, theme_type, tiles, animations })
    }
}

impl ThemeBuilder {
    pub fn add_terminal_terrain(self, tileset: &Tileset) -> Self {
        self.add_tile(tileset.get_tile_builder(0, TileType::Empty).build().unwrap())
            .add_tile(tileset.get_tile_builder(from_cp437('.'), TileType::Floor).build().unwrap())
            .add_tile(tileset.get_tile_builder(from_cp437('#'), TileType::Wall).build().unwrap())
    }

    pub fn add_terminal_features(self, tileset: &Tileset) -> Self { self }

    pub fn add_terminal_items(self, tileset: &Tileset) -> Self { self }

    pub fn add_terminal_actors(self, tileset: &Tileset) -> Self { self }
}
