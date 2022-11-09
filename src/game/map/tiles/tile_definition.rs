use crate::game::prelude::*;

pub struct TileDefinition {
    pub index: usize,
    pub atlas: Handle<TextureAtlas>,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}
