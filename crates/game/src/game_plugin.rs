use crate::prelude::*;

pub struct GamePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to
    /// [`state_construct`]
    pub state_construct: T,
    pub state_main_menu: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        let game_context = GameContext::default();

        app
            // set entry state
            .add_loopless_state(GameState::default())
            .insert_resource(TurnState::AwaitingInput)
            // SaveLoad
            .add_plugin(SaveLoadPlugin)
            // asset loading
            .add_plugin(RawPlugin {
                state_asset_load: GameState::AssetLoad(Load),
                state_asset_load_failure: GameState::AssetLoad(LoadFailure),
            })
            // common loading
            .add_plugin(CommonPlugin)
            // Create Camera
            .add_plugin(CameraPlugin::new(
                CameraSettings::new_dimensions(GRID_WIDTH as f32, GRID_HEIGHT as f32)
                    .with_id(CameraId::Map)
                    .with_position(Vec2::ZERO),
            ))
            // Map Rendering
            .add_plugin(MapPlugin::new(
                self.state_construct.clone(),
                self.state_running.clone(),
                TilemapSettings {
                    chunk_size: UVec2::new(GRID_WIDTH, GRID_HEIGHT),
                    tileset_folders: vec!["tilesets/".to_string()], /* TODO: Error checking with
                                                                     * asset_server.load_folder()
                                                                     * then builder pattern
                                                                     * adding folders */
                },
            ))
            .add_plugin(GameMapPlugin::new(
                self.state_construct.clone(),
                self.state_running.clone(),
            ))
            // Game Context
            .insert_resource(game_context)
            // `AppState::Initializing` is a buffer state to allow bevy plugins to initialize
            .add_enter_system(
                GameState::default(),
                switch_in_game_state!(GameState::default().next().unwrap()),
            )
            // UI
            .add_plugin(UiPlugin { state_main_menu: self.state_main_menu.clone() })
            // Spawner
            .add_plugin(SpawnerPlugin { state_running: self.state_running.clone() })
            // Player
            .add_plugin(PlayerPlugin { state_running: self.state_running.clone() });
    }
}
