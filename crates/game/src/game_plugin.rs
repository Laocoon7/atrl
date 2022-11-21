use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct GamePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to
    /// [`state_construct`]
    pub state_asset_load: T,
    pub state_asset_load_failure: T,

    pub state_construct: T,
    pub state_main_menu: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        let game_context = GameContext::default();

        // set entry state
        self.setup_states(app);

        app
            // Game Context
            .insert_resource(game_context)
            // SaveLoad
            .add_plugin(SaveLoadPlugin)
            // common loading
            .add_plugin(CommonPlugin)
            // Ecs Plugin (Systems)
            .add_plugin(EcsPlugin {
                state_running: self.state_running,
                turn_state_ticking: TurnState::Ticking,
            });

        self
            // Raw Files
            .add_raws(app, self.state_asset_load, self.state_asset_load_failure)
            // Create Camera
            .add_camera(app)
            // Map Rendering
            .add_map_rendering(self.state_construct, self.state_running, app);

        app
            // UI
            .add_plugin(UiPlugin {
                state_asset_load: self.state_asset_load,
                state_main_menu: self.state_main_menu,
            })
            // Spawner
            .add_plugin(SpawnerPlugin { state_construct: self.state_construct })
            // Player
            .add_plugin(PlayerPlugin { state_running: self.state_running });
    }
}

impl<T: StateNext> GamePlugin<T> {
    fn setup_states(self, app: &mut App) -> Self {
        app.add_loopless_state(GameState::Initializing)
            .insert_resource(TurnState::AwaitingInput)
            .add_enter_system(
                GameState::Initializing,
                switch_in_game_state!(GameState::AssetLoad(AssetLoadState::Load)),
            );

        self
    }

    fn add_camera(self, app: &mut App) -> Self {
        app.add_plugin(CameraPlugin::new(
            CameraSettings::new_dimensions(GRID_WIDTH as f32, GRID_HEIGHT as f32)
                .with_id(CameraId::Map)
                .with_position(Vec2::ZERO),
        ));

        self
    }

    fn add_raws(self, app: &mut App, state_asset_load: T, state_asset_load_failure: T) -> Self {
        let mut plugin = RawPlugin::new(state_asset_load, state_asset_load_failure);

        for path in get_tileset_paths() {
            plugin = plugin.add_tileset_file(path.as_str());
        }

        for path in get_font_paths() {
            plugin = plugin.add_font_file(path.as_str());
        }

        app.add_plugin(plugin);

        self
    }

    fn add_map_rendering(self, state_construct: T, state_running: T, app: &mut App) -> Self {
        app.add_plugin(MapPlugin::new(state_construct, state_running))
            .add_plugin(MapRendererPlugin::new([GRID_WIDTH, GRID_HEIGHT]));

        self
    }
}
