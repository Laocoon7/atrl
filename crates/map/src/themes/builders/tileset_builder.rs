use crate::prelude::*;

#[derive(Default)]
pub struct TilesetBuilder {
    pub file: Option<PathBuf>,
    pub name: Option<String>,

    pub image_handle: Option<Handle<Image>>,
    pub image_width: Option<f32>,
    pub image_height: Option<f32>,

    pub tile_width: Option<f32>,
    pub tile_height: Option<f32>,

    pub columns: Option<usize>,
    pub rows: Option<usize>,

    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,

    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
}

impl TilesetBuilder {
    /// Creates a new `TilesetBuilder`
    /// The following `.set_XXX` **must** be called prior to `.build()`:
    /// `.set_file()`
    /// `.set_name()`
    /// `.set_tile_width()`
    /// `.set_tile_height()`
    /// `.set_columns()`
    /// `.set_rows()`
    pub fn new() -> Self {
        Self {
            file: None,
            name: None,
            image_handle: None,
            image_width: None,
            image_height: None,
            tile_width: None,
            tile_height: None,
            columns: None,
            rows: None,
            padding_x: None,
            padding_y: None,
            offset_x: None,
            offset_y: None,
        }
    }

    #[must_use]
    pub fn set_file<Path: Into<PathBuf>>(&mut self, file: Path) -> &mut Self {
        self.file = Some(file.into());
        self
    }

    #[must_use]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_image_handle(&mut self, image_handle: &Handle<Image>) -> &mut Self {
        self.image_handle = Some(image_handle.clone());
        self
    }

    pub fn set_image_width(&mut self, image_width: f32) -> &mut Self {
        self.image_width = Some(image_width);
        self
    }

    pub fn set_image_height(&mut self, image_height: f32) -> &mut Self {
        self.image_height = Some(image_height);
        self
    }

    #[must_use]
    pub fn set_tile_width(&mut self, tile_width: f32) -> &mut Self {
        self.tile_width = Some(tile_width);
        self
    }

    #[must_use]
    pub fn set_tile_height(&mut self, tile_height: f32) -> &mut Self {
        self.tile_height = Some(tile_height);
        self
    }

    #[must_use]
    pub fn set_columns<U: Into<usize>>(&mut self, columns: U) -> &mut Self {
        let columns = columns.into();
        self.columns = Some(columns);
        self
    }

    #[must_use]
    pub fn set_rows<U: Into<usize>>(&mut self, rows: U) -> &mut Self {
        let rows = rows.into();
        self.rows = Some(rows);
        self
    }

    pub fn set_padding_x(&mut self, padding_x: f32) -> &mut Self {
        self.padding_x = Some(padding_x);
        self
    }

    pub fn set_padding_y(&mut self, padding_y: f32) -> &mut Self {
        self.padding_y = Some(padding_y);
        self
    }

    pub fn set_offset_x(&mut self, offset_x: f32) -> &mut Self {
        self.offset_x = Some(offset_x);
        self
    }

    pub fn set_offset_y(&mut self, offset_y: f32) -> &mut Self {
        self.offset_y = Some(offset_y);
        self
    }

    pub fn build(self) -> Result<Tileset> {
        let file = match self.file {
            Some(u) => match u.into_os_string().into_string() {
                Ok(u) => u,
                Err(_) => {
                    return Err(MyError::BuilderErrorGeneric(
                        "Tileset".to_string(),
                        "invalid filename".to_string(),
                    ))
                }
            },
            None => return Err(MyError::BuilderError("Tileset".to_string(), "file".to_string())),
        };

        let name = match self.name {
            Some(u) => u,
            None => return Err(MyError::BuilderError("Tileset".to_string(), "name".to_string())),
        };

        let tile_width = match self.tile_width {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Tileset".to_string(), "tile_width".to_string()))
            }
        };
        let tile_height = match self.tile_height {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Tileset".to_string(), "tile_height".to_string()))
            }
        };

        let columns = match self.columns {
            Some(u) => u,
            None => {
                return Err(MyError::BuilderError("Tileset".to_string(), "columns".to_string()))
            }
        };
        let rows = match self.rows {
            Some(u) => u,
            None => return Err(MyError::BuilderError("Tileset".to_string(), "rows".to_string())),
        };

        let padding_x = self.padding_x.unwrap_or(0.0);
        let padding_y = self.padding_y.unwrap_or(0.0);

        let offset_x = self.offset_x.unwrap_or(0.0);
        let offset_y = self.offset_y.unwrap_or(0.0);

        Ok(Tileset {
            file,
            name,
            tile_width,
            tile_height,
            columns,
            rows,
            padding_x,
            padding_y,
            offset_x,
            offset_y,
        })
    }
}
