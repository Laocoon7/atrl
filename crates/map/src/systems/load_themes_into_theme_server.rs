use crate::prelude::*;

const THEME_DIR: &str = "assets/themes/";

pub fn load_themes_into_theme_server(mut theme_server: ResMut<ThemeServer>) {
    theme_server.tilesets.clear();
    theme_server.themes.clear();
}
