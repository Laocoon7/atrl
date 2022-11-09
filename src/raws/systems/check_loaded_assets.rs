use crate::prelude::*;

pub fn check_progress(
    mut commands: Commands,
    splash_timer: Res<SplashTimer>,
    mut state: ResMut<CurrentGameState>,
    progress_counter: Option<Res<ProgressCounter>>,
) {
    if let Some(progress_counter) = progress_counter {
        let progress = progress_counter.progress();
        let progress_perc: f32 = progress.into();
        let progress_perc = if progress_perc.is_nan() { 0.0 } else { progress_perc };

        if progress_perc >= 1.0 && splash_timer.finished() {
            info!("Assets loaded and splash timer complete!");
            state.set_next(&mut commands);
        }
    }
}

pub fn check_loaded_assets(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    game_assets: Res<TextureAssets>,
    image_assets: Res<Assets<Image>>,
    mut state: ResMut<CurrentGameState>,
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
    state.set_next(&mut commands);
}
