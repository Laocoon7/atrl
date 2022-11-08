use crate::game::prelude::*;

pub struct RenderContext<'a> {
    renderer: &'a MapRenderer,
    actions: Vec<RenderAction>,
}

#[allow(dead_code)]
impl<'a> RenderContext<'a> {
    #[inline(always)]
    pub const fn new(renderer: &'a MapRenderer) -> Self { Self { renderer, actions: Vec::new() } }

    pub fn set_index(&mut self, layer: MapLayer, index: impl Point2d, image_index: usize) {
        if let Some(entity) = self.renderer.get_entity(layer, index) {
            self.actions.push(RenderAction::SetAtlasIndex(entity, image_index));
        }
    }

    pub fn set_atlas(
        &mut self,
        layer: MapLayer,
        index: impl Point2d,
        texture_atlas_handle: Handle<TextureAtlas>,
    ) {
        if let Some(entity) = self.renderer.get_entity(layer, index) {
            self.actions.push(RenderAction::SetAtlasHandle(entity, texture_atlas_handle));
        }
    }

    pub fn set_foreground_color(&mut self, layer: MapLayer, index: impl Point2d, color: Color) {
        if let Some(entity) = self.renderer.get_entity(layer, index) {
            self.actions.push(RenderAction::SetForegroundColor(entity, color));
        }
    }

    pub fn set_background_color(&mut self, layer: MapLayer, index: impl Point2d, color: Color) {
        if let Some(entity) = self.renderer.get_background_entity(layer, index) {
            self.actions.push(RenderAction::SetBackgroundColor(entity, color));
        }
    }

    pub fn finalize(self, commands: &mut Commands) {
        commands.spawn().insert(RenderActions { actions: self.actions });
    }
}
