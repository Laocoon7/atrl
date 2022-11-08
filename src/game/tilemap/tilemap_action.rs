use crate::prelude::*;

// TODO: Is this needed or should we switch names?
// https://rust-lang.github.io/rust-clippy/master/index.html#enum_variant_names
#[allow(clippy::enum_variant_names)]
pub enum TilemapAction {
    SetIndex(Entity, usize),
    SetAtlas(Entity, Handle<TextureAtlas>),
    SetColor(Entity, Color),
}
