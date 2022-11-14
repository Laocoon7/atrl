use crate::prelude::*;

pub fn create_renderer(
    mut commands: Commands,
    white_pixel: Res<WhitePixel>,
    game_assets: Res<TextureAssets>,
) {
    MapRenderer::build([GRID_WIDTH, GRID_HEIGHT], &white_pixel, &game_assets, &mut commands);
}
