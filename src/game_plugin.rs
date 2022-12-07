use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct GamePlugin<T> {
    /// Asset loading happens in this state. When it finishes it transitions to
    /// [`state_construct`]
    pub state_asset_load: T,
    pub state_asset_load_failure: T,

    pub state_construct: T,
    pub state_construct_setup: T,

    pub state_main_menu: T,
    pub state_running: T,
}

impl<T: StateNext> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) { self.setup_states(app).setup_resources(app).setup_game_plugins(app); }
}

impl<T: StateNext> GamePlugin<T> {
    fn setup_states(self, app: &mut App) -> Self {
        app.add_loopless_state(GameState::Initializing).add_enter_system(
            GameState::Initializing,
            switch_game_state!(self.state_asset_load),
        );
        self
    }

    fn setup_resources(self, app: &mut App) -> Self {
        app
            // Game Contexts
            .init_resource::<GameContext>()
            // Ai Context
            .init_resource::<AiContext>()
            // Turn Manager
            .init_resource::<TurnManager>()
            // Game Settings
            .init_resource::<GameSettings>()
            // Mouse Position
            .init_resource::<MousePosition>();
        self
    }

    fn setup_game_plugins(self, app: &mut App) -> Self {
        self
            // Raw Files
            .add_raws(app)
            // Create Camera
            .add_camera(app)
            // Map Rendering
            .add_map_plugins(app);

        app
            // Common plugin
            .add_plugin(CommonPlugin)
            // SaveLoad Plugin
            .add_plugin(SaveLoadPlugin)
            // Spawner
            .add_plugin(SpawnerPlugin { state_construct_setup: self.state_construct_setup })
            // Systems Plugin
            .add_plugin(SystemsPlugin {
                state_running: self.state_running,
            })
            // UI
            .add_plugin(UiPlugin {
                state_asset_load: self.state_asset_load,
                state_main_menu: self.state_main_menu
            });
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

    fn add_raws(self, app: &mut App) -> Self {
        let mut plugin = RawPlugin::new(
            self.state_asset_load,
            self.state_asset_load_failure,
            self.state_construct,
        );
        for path in get_tileset_paths() {
            plugin = plugin.add_tileset_file(path.as_str());
        }
        for path in get_font_paths() {
            plugin = plugin.add_font_file(path.as_str());
        }
        app.add_plugin(plugin);
        self
    }

    fn add_map_plugins(self, app: &mut App) -> Self {
        app.add_plugin(MapPlugin::new(self.state_construct, self.state_running))
            .add_plugin(MapRendererPlugin::new([GRID_WIDTH, GRID_HEIGHT]));
        self
    }
}
