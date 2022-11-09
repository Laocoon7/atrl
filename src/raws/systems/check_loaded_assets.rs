use crate::prelude::*;

pub fn check_loaded_assets(
    game_assets: Res<GameAssets>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    image_assets: Res<Assets<Image>>,
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
}
