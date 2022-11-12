use crate::game::prelude::internal::*;
use crate::prelude::*;

#[derive(Deserialize)]
pub struct TextureAtlasTemplate {
    pub file: String,
    pub rows: usize,
    pub columns: usize,
    pub tile_width: f32,
    pub tile_height: f32,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,
    pub tile_templates: Vec<TileTemplate>,
}
