use crate::game::prelude::*;

use bevy::ecs::system::SystemState;

const DEFAULT_TEXTURE_ATLAS_PATH: &str = "tilesets/terminal8x8_transparent.png";
const DEFAULT_TEXTURE_ATLAS_TILE_WIDTH: f32 = 8.0;
const DEFAULT_TEXTURE_ATLAS_TILE_HEIGHT: f32 = 8.0;
const DEFAULT_TEXTURE_ATLAS_COLUMNS: usize = 16;
const DEFAULT_TEXTURE_ATLAS_ROWS: usize = 16;
const DEFAULT_TEXTURE_ATLAS_PADDING_X: f32 = 0.0;
const DEFAULT_TEXTURE_ATLAS_PADDING_Y: f32 = 0.0;
const DEFAULT_TEXTURE_ATLAS_OFFSET_X: f32 = 0.0;
const DEFAULT_TEXTURE_ATLAS_OFFSET_Y: f32 = 0.0;

pub struct DefaultAssets {
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

impl FromWorld for DefaultAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<AssetServer>, ResMut<Assets<TextureAtlas>>)> =
            SystemState::new(world);
        let (asset_server, mut atlases) = system_state.get_mut(world);

        let image_handle = asset_server.load(DEFAULT_TEXTURE_ATLAS_PATH);
        let atlas = TextureAtlas::from_grid_with_padding(
            image_handle,
            Vec2::new(DEFAULT_TEXTURE_ATLAS_TILE_WIDTH, DEFAULT_TEXTURE_ATLAS_TILE_HEIGHT),
            DEFAULT_TEXTURE_ATLAS_COLUMNS,
            DEFAULT_TEXTURE_ATLAS_ROWS,
            Vec2::new(DEFAULT_TEXTURE_ATLAS_PADDING_X, DEFAULT_TEXTURE_ATLAS_PADDING_Y),
            Vec2::new(DEFAULT_TEXTURE_ATLAS_OFFSET_X, DEFAULT_TEXTURE_ATLAS_OFFSET_Y),
        );
        let texture_atlas_handle = atlases.add(atlas);

        println!("Loaded DefaultAssets!");
        DefaultAssets { texture_atlas_handle }
    }
}
