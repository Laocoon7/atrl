use crate::app_settings::*;
use crate::prelude::*;

pub fn create_tilemaps(mut commands: Commands, game_assets: Res<GameAssets>) {
    let texture_atlas_handle = game_assets.terminal8x8_atlas.clone();

    Tilemap::build(
        TilemapId::Terrain,
        [GRID_WIDTH, GRID_HEIGHT],
        TilemapId::Terrain,
        &texture_atlas_handle,
        &mut commands,
        &mut |(_x, _y)| TileDefinition {
            background_index: 236,
            background_color: Color::BLACK,
            foreground_index: 0,
            foreground_color: Color::WHITE,
        },
    );

    Tilemap::build(
        TilemapId::Features,
        [GRID_WIDTH, GRID_HEIGHT],
        TilemapId::Features,
        &texture_atlas_handle,
        &mut commands,
        &mut |(_x, _y)| TileDefinition {
            background_index: 0,
            background_color: Color::BLACK,
            foreground_index: 0,
            foreground_color: Color::WHITE,
        },
    );

    Tilemap::build(
        TilemapId::Items,
        [GRID_WIDTH, GRID_HEIGHT],
        TilemapId::Items,
        &texture_atlas_handle,
        &mut commands,
        &mut |(_x, _y)| TileDefinition {
            background_index: 0,
            background_color: Color::BLACK,
            foreground_index: 0,
            foreground_color: Color::WHITE,
        },
    );
}
