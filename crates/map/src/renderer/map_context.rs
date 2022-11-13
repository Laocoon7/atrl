use crate::prelude::*;

pub struct MapContext<'a> {
    renderer: &'a mut MapRenderer,
    actions: Vec<RenderAction>,
}

impl<'a> MapContext<'a> {
    #[inline(always)]
    pub fn new(renderer: &'a mut MapRenderer) -> Self { Self { renderer, actions: Vec::new() } }

    pub fn set_theme(&mut self, theme_name: &str) -> &mut Self {
        self.actions.push(RenderAction::SetTheme(theme_name.to_string()));
        self
    }

    pub fn set_tile<T: Into<u16>>(&mut self, index: impl Point2d, tile_type: T) -> &mut Self {
        self.actions.push(RenderAction::SetTile(index.as_uvec2(), tile_type.into()));
        self
    }

    pub fn clear_tile<T: Into<u16>>(&mut self, index: impl Point2d) -> &mut Self {
        self.actions.push(RenderAction::ClearTile(index.as_uvec2()));
        self
    }

    pub(crate) fn set_raw(
        &mut self,
        foreground_entity: &Entity,
        texture_atlas_handle: &Handle<TextureAtlas>,
        index: usize,
        foreground_color: &Color,
        background_entity: &Entity,
        background_color: &Color,
    ) -> &mut Self {
        self.actions.push(RenderAction::SetRaw(
            foreground_entity.clone(),
            texture_atlas_handle.clone(),
            index,
            foreground_color.clone(),
            background_entity.clone(),
            background_color.clone(),
        ));
        self
    }

    pub fn finalize(self) { self.renderer.add_actions(self.actions); }
}
