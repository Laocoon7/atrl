use crate::prelude::*;

pub enum RenderAction {
    SetTheme(String),
    SetTile(UVec2, u16),
    ClearTile(UVec2),
}
