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

        // set entry state
        Self::setup_states(app);

        app.insert_resource(TurnState::AwaitingInput)
            // SaveLoad
            .add_plugin(SaveLoadPlugin)
            // asset loading
            .add_plugin(RawPlugin {
                state_asset_load: GameState::AssetLoad(Load),
                state_asset_load_failure: GameState::AssetLoad(LoadFailure),
            })
            // common loading
            .add_plugin(CommonPlugin);
        // Create Camera
        Self::add_camera(app);
        // Map Rendering
        Self::add_map_renderer(self.state_construct.clone(), self.state_running.clone(), app);
        Self::add_game_map(self.state_construct.clone(), self.state_running.clone(), app);

        app.add_plugin(EcsPlugin {
            state_running: self.state_running.clone(),
            turn_state_ticking: TurnState::Ticking,
        })
        // Game Context
        .insert_resource(game_context)
        // UI
        .add_plugin(UiPlugin { state_main_menu: self.state_main_menu.clone() })
        // Spawner
        .add_plugin(SpawnerPlugin { state_running: self.state_running.clone() })
        // Player
        .add_plugin(PlayerPlugin { state_running: self.state_running.clone() });
    }
}

impl<T: StateNext> GamePlugin<T> {
    fn setup_states(app: &mut App) {
        app.add_loopless_state(GameState::default()).add_enter_system(
            GameState::default(),
            switch_in_game_state!(GameState::AssetLoad(AssetLoadState::Load)),
        );
    }

    fn add_camera(app: &mut App) {
        app.add_plugin(CameraPlugin::new(
            CameraSettings::new_dimensions(GRID_WIDTH as f32, GRID_HEIGHT as f32)
                .with_id(CameraId::Map)
                .with_position(Vec2::ZERO),
        ));
    }

    fn add_map_renderer(state_construct: T, state_running: T, app: &mut App) {
        app.add_plugin(
            MapRendererPlugin::new(state_construct, state_running, [GRID_WIDTH, GRID_HEIGHT])
                //.add_tileset_file("path/to/file.ron")
                .add_tileset_folder("tilesets/"),
        );
    }

    fn add_game_map(state_construct: T, state_running: T, app: &mut App) {
        app.add_plugin(MapPlugin::new(state_construct, state_running));
    }
}
