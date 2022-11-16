use crate::prelude::*;

#[derive(Resource)]
pub(crate) struct LoadedTilesets {
    pub tileset_folders: Vec<String>,
    pub handles: Vec<HandleUntyped>,
}

impl LoadedTilesets {
    pub fn new(tileset_file_paths: &Vec<String>) -> Self {
        let tileset_file_paths = tileset_file_paths.clone();
        Self { tileset_folders: tileset_file_paths, handles: Vec::new() }
    }
}
