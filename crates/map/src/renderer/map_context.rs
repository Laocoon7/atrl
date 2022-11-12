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

    pub fn finalize(self) { self.renderer.add_actions(self.actions); }
}
