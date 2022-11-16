use crate::prelude::*;

#[derive(Resource)]
pub(crate) struct LoadedTilesets {
    pub tileset_folders: Vec<String>,
    pub handles: Vec<HandleUntyped>,
}

impl LoadedTilesets {
    pub fn new(tileset_file_paths: &[String]) -> Self {
        Self { tileset_folders: tileset_file_paths.to_vec(), handles: Vec::new() }
    }
}
