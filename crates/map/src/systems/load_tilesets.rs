use crate::prelude::*;

pub(crate) fn load_tilesets(
    asset_server: Res<AssetServer>,
    mut loaded_tilesets: ResMut<LoadedTilesets>,
    _state: Res<CurrentGameState>,
) {
    let base_path = FileAssetIo::get_base_path();
    let asset_path = base_path.join(ASSETS_PATH);
    let tilesets_path = asset_path.join(TILESETS_PATH);
    let default_file = tilesets_path.join(DEFAULT_TILESET_NAME).with_extension(RON_EXT);
    loaded_tilesets.default_handle = Some(asset_server.load(default_file.clone()));

    match get_files_with_extension(&tilesets_path, RON_EXT) {
        Ok(files) => {
            for file in files {
                let p = tilesets_path.join(file);
                if p == default_file {
                    continue;
                }

                let handle = asset_server.load(p);
                loaded_tilesets.other_handles.push(handle);
            }
        }
        Err(e) => error!("Error loading tileset: {}", e),
    }
}
