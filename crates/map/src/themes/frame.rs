use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub tileset_name: String,

    pub xy: Option<(usize, usize)>,
    pub index: Option<usize>,

    pub foreground_color: Option<ColorDefinition>,
    pub background_color: Option<ColorDefinition>,
}

//impl_serialized_object_for!(Frame);
