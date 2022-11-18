use crate::prelude::*;

pub(crate) fn load_tilesets(
    asset_server: Res<AssetServer>,
    mut loaded_tilesets: ResMut<LoadedTilesets>,
) {
    let mut handles = Vec::new();
    for path in loaded_tilesets.tileset_folders.iter() {
        match asset_server.load_folder(path) {
            Ok(mut vec) => {
                for handle in vec.drain(..) {
                    handles.push(handle.typed::<Tileset>());
                }
            }
            Err(e) => error!("{}", e),
        }
    }
    for path in loaded_tilesets.tileset_files.iter() {
        let handle = asset_server.load(path);
        handles.push(handle);
    }
    loaded_tilesets.handles = handles;
}
