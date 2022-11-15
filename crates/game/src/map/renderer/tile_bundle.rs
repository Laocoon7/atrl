use crate::prelude::*;

#[derive(Component, Default)]
pub struct RenderTile;

#[derive(Bundle, Default)]
pub struct RenderTileBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub render_tile: RenderTile,
}

#[derive(Bundle, Default)]
pub struct RenderTileBundleSprite {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub render_tile: RenderTile,
}
