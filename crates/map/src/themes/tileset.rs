use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub file: String,
    pub name: String,

    pub tile_width: f32,
    pub tile_height: f32,

    pub columns: usize,
    pub rows: usize,

    pub padding_x: f32,
    pub padding_y: f32,

    pub offset_x: f32,
    pub offset_y: f32,
}

impl_serialized_object_for!(Tileset);
