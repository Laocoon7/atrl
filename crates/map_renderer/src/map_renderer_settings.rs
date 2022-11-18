use crate::prelude::*;

pub(crate) struct MapRendererSettings {
    pub chunk_size: UVec2,
    pub tileset_file_paths: Vec<String>,
    pub tileset_folder_paths: Vec<String>,
}

impl MapRendererSettings {
    pub fn new(chunk_size: impl Size2d) -> Self {
        Self {
            chunk_size: chunk_size.as_uvec2(),
            tileset_file_paths: Vec::new(),
            tileset_folder_paths: Vec::new(),
        }
    }
}
