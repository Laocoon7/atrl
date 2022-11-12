use crate::prelude::*;

pub struct TileBuilder {
    pub tileset_name: Option<String>,

    pub tile_type: Option<u16>,

    pub xy: Option<(usize, usize)>,
    pub index: Option<usize>,

    pub foreground_color: Option<ColorDefinition>,
    pub background_color: Option<ColorDefinition>,
}

impl TileBuilder {
    /// Creates a new `TileBuilder`
    /// The following `.set_XXX` **must** be called prior to `.build()`:
    /// `.set_tileset_name()`
    /// `.set_tile_type()`
    /// `.set_xy()` or `.set_index`
    pub fn new() -> Self {
        Self {
            tileset_name: None,
            tile_type: None,
            xy: None,
            index: None,
            foreground_color: None,
            background_color: None,
        }
    }

    pub fn set_tileset_name(&mut self, tileset_name: &str) -> &mut Self {
        self.tileset_name = Some(tileset_name.to_string());
        self
    }

    pub fn set_tile_type<T: Into<u16>>(&mut self, tile_type: T) -> &mut Self {
        self.tile_type = Some(tile_type.into());
        self
    }

    pub fn set_xy<X: Into<usize>, Y: Into<usize>>(&mut self, xy: (X, Y)) -> &mut Self {
        self.xy = Some((xy.0.into(), xy.1.into()));
        self.index = None;
        self
    }

    pub fn set_index<I: Into<usize>>(&mut self, index: I) -> &mut Self {
        self.index = Some(index.into());
        self.xy = None;
        self
    }

    pub fn set_foreground_color(&mut self, color_definition: ColorDefinition) -> &mut Self {
        self.foreground_color = Some(color_definition);
        self
    }

    pub fn set_background_color(&mut self, color_definition: ColorDefinition) -> &mut Self {
        self.background_color = Some(color_definition);
        self
    }

    pub fn build(self) -> Result<Tile> {
        let tileset_name = match self.tileset_name {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Tile".to_string(), "tileset_name".to_string()))
            }
        };

        let tile_type = match self.tile_type {
            Some(u) => u,
            None => return Err(MyError::BuilderError("Tile".to_string(), "tile_type".to_string())),
        };

        let xy = self.xy;
        let index = self.index;
        if xy.is_none() && index.is_none() {
            return Err(MyError::BuilderError("Tile".to_string(), "xy` and `index".to_string()));
        }

        let foreground_color = self.foreground_color;
        let background_color = self.background_color;

        Ok(Tile { tileset_name, tile_type, xy, index, foreground_color, background_color })
    }
}
