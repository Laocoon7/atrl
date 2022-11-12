use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub tile_type: u16,

    pub frames_per_second: f32,

    pub frames: Vec<Frame>,
}

//impl_serialized_object_for!(Animation);
