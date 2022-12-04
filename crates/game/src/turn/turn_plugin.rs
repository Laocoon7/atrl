use crate::prelude::*;

// Handles the ProccessEvents Stage
#[derive(Copy, Clone)]
pub struct ProccessTurnsPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> ProccessTurnsPlugin<T> {
    fn setup_turn_stages(self, app: &mut App) -> Self {
        app.add_stage_after(
            AtrlStage::AIThinking,
            AtrlStage::ProcessTurns,
            SystemStage::parallel(),
        );
        self
    }

    fn setup_turn_plugins(self, app: &mut App) -> Self {
        app
        // Player
        .add_plugin(PlayerPlugin {
            state_running: self.state_running,
        });
        self
    }
}

impl<T: StateNext> Plugin for ProccessTurnsPlugin<T> {
    fn build(&self, app: &mut App) {
        self.setup_turn_stages(app).setup_turn_plugins(app);

        app.add_system_set_to_stage(
            AtrlStage::ProcessTurns,
            ConditionSet::new().run_in_state(self.state_running).with_system(perform_turns).into(),
        );
    }
}
