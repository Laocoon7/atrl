use crate::prelude::*;
#[derive(Resource)]
pub struct LoadedTilesets {
    pub tileset_files: Vec<String>,
    pub tileset_folders: Vec<String>,
    pub handles: Vec<Handle<Tileset>>,
}
impl LoadedTilesets {
    pub fn new(settings: &AssetSettings) -> Self {
        let tileset_files = settings.tileset_file_paths.clone();
        let tileset_folders = settings.tileset_folder_paths.clone();
        Self {
            tileset_files,
            tileset_folders,
            handles: Vec::new(),
        }
    }
}
