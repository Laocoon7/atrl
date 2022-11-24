use crate::prelude::*;
#[derive(Default,)]
pub struct AssetSettings {
    pub tileset_file_paths: Vec<String,>,
    pub tileset_folder_paths: Vec<String,>,

    pub font_file_paths: Vec<String,>,
    pub font_folder_paths: Vec<String,>,
}
