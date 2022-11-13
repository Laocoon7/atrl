const THEME_DIR: &str = "assets/themes/";
const TILESET_DIR: &str = "assets/themes/tilesets/";
const DEFAULT: &str = "default/";

pub fn rebuild_default_theme() {
    if let Ok(tileset) = get_default_tileset() {
        let terrain_result = ThemeBuilder::new()
            .set_name("default_terrain")
            .set_theme_type(ThemeType::Terrain)
            .add_terminal_terrain(&tileset)
            .build();
        
        let features_result = ThemeBuilder::new()
            .set_name("default_features")
            .set_theme_type(ThemeType::Feature)
            .add_terminal_features(&tileset)
            .build();
        
        let items_result = ThemeBuilder::new()
            .set_name("default_items")
            .set_theme_type(ThemeType::Item)
            .add_terminal_items(&tileset)
            .build();
        
        let actors_result = ThemeBuilder::new()
            .set_name("default_actors")
            .set_theme_type(ThemeType::Actor)
            .add_terminal_actors(&tileset)
            .build();

        if let Err(e) = tileset.to_file_ron(format!("{}{}.ron", TILESET_DIR, tileset.name)) {
            error!("{}", e);
        }

        serialize_theme(&terrain_result);
        serialize_theme(&features_result);
        serialize_theme(&items_result);
        serialize_theme(&actors_result);
    }
}

fn get_default_tileset() -> Result<Tileset> {
    TilesetBuilder::new()
        .set_file("terminal8x8_transparent.png")
        .set_name("terminal8x8")
        .set_tile_width(8.0)
        .set_tile_height(8.0)
        .set_columns(16usize)
        .set_rows(16usize)
        .build()
}

fn serialize_theme(result: &Result<Theme>) {
    match result {
        Ok(theme) => {
            if let Err(e) = theme.to_file_ron(format!("{}{}.ron", THEME_DIR, theme.name)) {
                error!("{}", e);
            }
        },
        Err(e) => error!("{}", e),
    }
}