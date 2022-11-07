use crate::game::prelude::*;

#[derive(Component)]
pub struct TilemapContext {
    pub actions: Vec<TilemapAction>,
}
