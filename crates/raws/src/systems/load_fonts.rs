use crate::prelude::*;

pub fn load_game_fonts<T: StateNext>(
    fail_state: T,
    mut commands: Commands,
    state: Res<CurrentGameState>,
    asset_server: Res<AssetServer>,
    mut loaded_tilesets: ResMut<LoadedFonts>,
) {
    let mut handles = Vec::new();
    for path in loaded_tilesets.font_folders.iter() {
        match asset_server.load_folder(path) {
            Ok(vec) => {
                for handle in vec.into_iter() {
                    handles.push(handle.typed::<Font>());
                }
            }
            Err(e) => {
                error!("Failed loading fonts: {}", e);
                state.go_to(&mut commands, fail_state);
            }
        }
    }

    loaded_tilesets.font_files.iter().for_each(|path| {
        let handle = asset_server.load(path);
        handles.push(handle);
    });
    loaded_tilesets.handles = handles;
}
