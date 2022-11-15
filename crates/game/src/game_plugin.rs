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
        app
            // set entry state
            .add_loopless_state(GameState::default())
            .insert_resource(TurnState::AwaitingInput)
            // asset loading
            .add_plugin(RawPlugin {
                state_asset_load: GameState::AssetLoad(Load),
                state_asset_load_failure: GameState::AssetLoad(LoadFailure),
            })
            // common loading
            .add_plugin(CommonPlugin)
            // Game Context
            .init_resource::<GameContext>()
            // Create Camera
            .add_plugin(
                CameraPlugin::new().add_camera(
                    CameraSettings::new_dimensions(GRID_WIDTH as f32, GRID_HEIGHT as f32)
                        .with_id(CameraId::Map)
                        .with_position(Vec2::ZERO),
                ),
            )
            // Map Rendering
            .add_plugin(MapPlugin {
                state_running: self.state_running.clone(),
                state_construct: self.state_construct.clone(),
            })
            // `AppState::Initializing` is a buffer state to allow bevy plugins to initialize
            .add_enter_system(
                GameState::default(),
                switch_in_game_state!(GameState::default().next().unwrap()),
            )
            // Copy Map to Map Renderer
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .with_system(draw_map)
                    .into(),
            )
            // UI
            .add_plugin(UiPlugin { state_main_menu: self.state_main_menu.clone() })
            // Spawner
            .add_plugin(SpawnerPlugin { state_running: self.state_running.clone() })
            // Player
            .add_plugin(PlayerPlugin { state_running: self.state_running.clone() });
    }
}
