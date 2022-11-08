use crate::app_settings::*;
use crate::prelude::*;

pub fn create_renderer(mut commands: Commands, game_assets: Res<GameAssets>) {
    MapRenderer::build([GRID_WIDTH, GRID_HEIGHT], &game_assets, &mut commands);
}
