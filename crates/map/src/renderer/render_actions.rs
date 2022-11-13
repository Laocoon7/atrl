use crate::prelude::*;

pub enum RenderAction {
    SetTheme(String),
    SetTile(UVec2, u16),
    ClearTile(UVec2),
    SetRaw(Entity, Handle<TextureAtlas>, usize, Color, Entity, Color),
}
