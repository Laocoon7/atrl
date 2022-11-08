use crate::game::prelude::*;

#[derive(Component)]
pub struct RenderActions {
    pub actions: Vec<RenderAction>,
}

// feel free to add this back if needed
#[allow(clippy::enum_variant_names)]
pub enum RenderAction {
    SetAtlasIndex(Entity, usize),
    SetAtlasHandle(Entity, Handle<TextureAtlas>),
    SetForegroundColor(Entity, Color),
    SetBackgroundColor(Entity, Color),
}
