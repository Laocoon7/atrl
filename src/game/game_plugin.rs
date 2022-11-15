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
            // common loading
            .add_plugin(CommonPlugin)
            // Game Context
            .init_resource::<GameContext>()
            // Create Camera
            .add_plugin(CameraPlugin)
            // Map Rendering
            .add_plugin(MapPlugin {
                state_running: self.state_running.clone(),
                state_construct: self.state_construct.clone(),
            })
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
