use crate::prelude::*;
pub struct RawPlugin<T> {
    pub state_asset_load: T,
    pub state_asset_load_failure: T,
    settings: AssetSettings,
}
impl<T: StateNext> RawPlugin<T> {
    pub fn new(state_asset_load: T, state_asset_load_failure: T) -> Self {
        Self {
            state_asset_load,
            state_asset_load_failure,
            settings: AssetSettings::default(),
        }
    }

    pub fn add_tileset_file(mut self, file: &str) -> Self {
        self.settings.tileset_file_paths.push(file.to_string());
        self
    }

    pub fn add_tileset_folder(mut self, folder: &str) -> Self {
        self.settings.tileset_folder_paths.push(folder.to_string());
        self
    }

    pub fn add_font_file(mut self, file: &str) -> Self {
        self.settings.font_file_paths.push(file.to_string());
        self
    }

    pub fn add_font_folder(mut self, folder: &str) -> Self {
        self.settings.font_folder_paths.push(folder.to_string());
        self
    }
}
impl<T: StateNext> Plugin for RawPlugin<T> {
    fn build(&self, app: &mut App) {
        let failure_state = self.state_asset_load_failure;
        app
            // bevy_tileset
            .add_plugin(TilesetPlugin::default(),)
            // Progress Tracker
            .add_plugin(ProgressPlugin::new(self.state_asset_load,),)
            // hold `Handle<Tileset>`s so they don't get unloaded
            .insert_resource(LoadedTilesets::new(&self.settings,),)
            // hold `Handle<Font>`s so they don't get unloaded
            .insert_resource(LoadedFonts::new(&self.settings,),);
        // Load Assets
        app.add_enter_system_set(
            self.state_asset_load,
            ConditionSet::new()
                    // Fonts + Tilesets
                    .with_system(
                        move |commands: Commands,
                              state: Res<CurrentGameState,>,
                              asset_server: Res<AssetServer,>,
                              loaded_fonts: ResMut<LoadedFonts,>| {
                            load_game_fonts(
                                failure_state,
                                commands,
                                state,
                                asset_server,
                                loaded_fonts,
                            );
                        },
                    )
                    // Textures
                    .with_system(load_game_textures,)
                    // Tilesets
                    .with_system(
                        move |commands: Commands,
                              state: Res<CurrentGameState,>,
                              asset_server: Res<AssetServer,>,
                              loaded_tilesets: ResMut<LoadedTilesets,>| {
                            load_game_tilesets(
                                failure_state,
                                commands,
                                state,
                                asset_server,
                                loaded_tilesets,
                            );
                        },
                    )
                    .into(),
        )
        .add_system(check_loaded_assets.run_in_state(self.state_asset_load));
    }
}
