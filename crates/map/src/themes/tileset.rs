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

impl Tileset {
    pub fn get_tile_builder<T: Into<u16>>(&self, index: usize, tile_type: T) -> TileBuilder {
        TileBuilder::new().set_tileset_name(&self.name).set_tile_type(tile_type).set_index(index)
    }

    pub fn get_frame_builder<T: Into<u16>>(&self, index: usize) -> FrameBuilder {
        FrameBuilder::new().set_tileset_name(&self.name).set_index(index)
    }
}

impl_serialized_object_for!(Tileset);
