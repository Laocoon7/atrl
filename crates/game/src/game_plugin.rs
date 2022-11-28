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
    fn build(&self, app: &mut App) {
        let game_context = GameContext::default();

        // set entry state
        self.setup_states(app);

        app
            // Game Context
            .insert_resource(game_context)
            // Turn Manager
            .init_resource::<TurnManager>()
            // SaveLoad
            .add_plugin(SaveLoadPlugin)
            // common plugin
            .add_plugin(CommonPlugin)
            // Ecs Plugin (Systems)
            .add_plugin(EcsPlugin {
                state_running: self.state_running,
                turn_state_ticking: TurnState::Ticking,
            });

        self
            // Raw Files
            .add_raws(app)
            // Create Camera
            .add_camera(app)
            // Map Rendering
            .add_map_plugins(app);

        app
            // UI
            .add_plugin(UiPlugin {
                state_asset_load: self.state_asset_load,
                state_main_menu: self.state_main_menu
            })
            // Spawner
            .add_plugin(SpawnerPlugin { state_construct_setup: self.state_construct_setup });
    }
}

impl<T: StateNext> GamePlugin<T> {
    fn setup_states(self, app: &mut App) -> Self {
        app.add_loopless_state(GameState::Initializing)
            .insert_resource(TurnState::AwaitingInput)
            .add_enter_system(
                GameState::Initializing,
                switch_in_game_state!(self.state_asset_load),
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
