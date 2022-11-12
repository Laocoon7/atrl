use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub theme_type: u8,

    pub tiles: Vec<Tile>,
    pub animations: Vec<Animation>,
}

impl_serialized_object_for!(Theme);
