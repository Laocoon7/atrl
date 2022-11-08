use crate::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct GameAssets {
    // Any file that can be loaded and turned into a texture atlas
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 16, rows = 16))]
    #[asset(path = "tilesets/terminal8x8_transparent.png")]
    pub terminal8x8_atlas: Handle<TextureAtlas>,

    // Any file that can be loaded and turned into a Image
    #[asset(path = "tilesets/terminal8x8_transparent.png")]
    pub terminal8x8: Handle<Image>,

    #[asset(path = "images/white_pixel.png")]
    pub white_pixel: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(key = "julia_mono", collection(typed))]
    pub julia_mono: Vec<Handle<Font>>,
}

pub struct RawPlugin<T> {
    pub state_asset_load: T,
    pub state_construct: T,
}

impl<T: StateNext> Plugin for RawPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(self.state_asset_load.clone())
                .continue_to_state(self.state_construct.clone())
                .with_collection::<GameAssets>()
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "collection.assets",
                ])
                .with_collection::<FontAssets>(),
        )
        .add_enter_system(self.state_construct.clone(), expectations);
    }
}

fn expectations(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
    mut state: ResMut<CurrentGameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    use bevy::asset::LoadState;

    info!("Done loading the collection. Checking expectations...");

    // Did fonts load?
    for handle in font_assets.julia_mono.iter() {
        assert_eq!(asset_server.get_load_state(handle.clone()), LoadState::Loaded);
    }

    // Did `terminal8x8 image` load?
    image_assets
        .get(&game_assets.terminal8x8)
        .expect("terminal8x8 should be added to its assets resource.");
    assert_eq!(asset_server.get_load_state(game_assets.terminal8x8.clone()), LoadState::Loaded);

    // Did `terminal8x8_atlas` load?
    let atlas = texture_atlases
        .get(&game_assets.terminal8x8_atlas)
        .expect("terminal8x8_atlas should be added to its assets resource.");
    assert_eq!(asset_server.get_load_state(atlas.texture.clone()), LoadState::Loaded);

    info!("Everything looks good! Switching to the next state.");
    state.set_next(&mut commands);
}
