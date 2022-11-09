use crate::game::prelude::*;

#[derive(Deserialize)]
pub struct TextureAtlasTemplate {
    pub file: String,
    pub tile_width: f32,
    pub tile_height: f32,
    pub columns: usize,
    pub rows: usize,
    pub padding_x: Option<f32>,
    pub padding_y: Option<f32>,
    pub offset_x: Option<f32>,
    pub offset_y: Option<f32>,

    pub tile_templates: Vec<TileTemplate>,
}
