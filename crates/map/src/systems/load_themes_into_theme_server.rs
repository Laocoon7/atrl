use crate::prelude::*;

const THEME_DIR: &str = "assets/themes/";
const TILESET_DIR: &str = "assets/themes/tilesets/";
const DEFAULT: &str = "default/";

pub fn load_themes_into_theme_server(mut theme_server: ResMut<ThemeServer>) {
    theme_server.tilesets.clear();
    theme_server.themes.clear();
}
