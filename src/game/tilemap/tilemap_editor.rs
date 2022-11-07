use crate::game::prelude::*;

pub struct TilemapEditor<'a> {
    tilemap: &'a Tilemap,
    actions: Vec<TilemapAction>,
}

#[allow(dead_code)]
impl<'a> TilemapEditor<'a> {
    pub fn new(tilemap: &'a Tilemap) -> Self { Self { tilemap, actions: Vec::new() } }

    pub fn set_index(&mut self, x: u32, y: u32, index: usize) {
        if let Some(entity) = self.tilemap.get_entity(x, y) {
            self.actions.push(TilemapAction::SetIndex(entity, index));
        }
    }

    pub fn set_atlas(&mut self, x: u32, y: u32, texture_atlas_handle: Handle<TextureAtlas>) {
        if let Some(entity) = self.tilemap.get_entity(x, y) {
            self.actions.push(TilemapAction::SetAtlas(entity, texture_atlas_handle));
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        if let Some(entity) = self.tilemap.get_entity(x, y) {
            self.actions.push(TilemapAction::SetColor(entity, color));
        }
    }

    pub fn finalize(self, commands: &mut Commands) {
        commands.spawn().insert(TilemapContext { actions: self.actions });
    }
}
