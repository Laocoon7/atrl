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
                    handles.push(handle);
                }
            }
            Err(e) => error!("{}", e),
        }
    }
    loaded_tilesets.handles = handles;
}
