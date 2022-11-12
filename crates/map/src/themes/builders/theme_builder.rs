use crate::prelude::*;

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

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_theme_type<T: Into<u8>>(&mut self, theme_type: T) -> &mut Self {
        self.theme_type = Some(theme_type.into());
        self
    }

    pub fn add_tile(&mut self, tile: Tile) -> &mut Self {
        self.tiles.push(tile);
        self
    }

    pub fn add_animation(&mut self, animation: Animation) -> &mut Self {
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
