use crate::game::prelude::*;

pub enum TilemapAction {
    SetIndex(Entity, usize),
    SetAtlas(Entity, Handle<TextureAtlas>),
    SetColor(Entity, Color),
}
