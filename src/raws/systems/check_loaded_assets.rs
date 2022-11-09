use crate::prelude::*;

pub fn check_loaded_assets(
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    game_assets: Res<TextureAssets>,
    image_assets: Res<Assets<Image>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    use bevy::asset::LoadState;

    info!("Done loading the collection. Checking expectations...");

    // Did fonts load?
    [
        font_assets.julia_mono.clone(),
        font_assets.julia_mono_bold.clone(),
        font_assets.julia_mono_medium.clone(),
    ]
    .iter()
    .for_each(|f| {
        assert_eq!(asset_server.get_load_state(f.clone()), LoadState::Loaded);
    });

    // Did `white_pixel image` load?
    image_assets
        .get(&game_assets.white_pixel)
        .expect("white_pixel should be added to its assets resource.");
    assert_eq!(asset_server.get_load_state(game_assets.white_pixel.clone()), LoadState::Loaded);

    // Did `terminal8x8_atlas` load?
    let atlas = texture_atlases
        .get(&game_assets.terminal8x8_atlas)
        .expect("terminal8x8_atlas should be added to its assets resource.");
    assert_eq!(asset_server.get_load_state(atlas.texture.clone()), LoadState::Loaded);

    info!("Everything looks good! Switching to the next state.");
}
