use crate::prelude::*;
// #[derive(Resource)]
// pub struct FontAssets {
//     pub julia_mono: Handle<Font>,
//     pub julia_mono_bold: Handle<Font>,
//     pub julia_mono_medium: Handle<Font>,
// }
use crate::prelude::*;
#[derive(Resource)]
pub struct LoadedFonts {
    pub font_files: Vec<String>,
    pub font_folders: Vec<String>,
    pub handles: Vec<Handle<Font>>,
}
impl LoadedFonts {
    pub fn new(settings: &AssetSettings) -> Self {
        let font_files = settings.tileset_file_paths.clone();
        let font_folders = settings.tileset_folder_paths.clone();
        Self {
            font_files,
            font_folders,
            handles: Vec::new(),
        }
    }
}
