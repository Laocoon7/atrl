use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct AIPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> AIPlugin<T> {
    fn setup_ai_stages(self, app: &mut App) -> Self {
        use CoreStage::*;
        app.add_stage_after(PreUpdate, AtrlStage::AIThinking, SystemStage::parallel());
        self
    }
}

impl<T: StateNext> Plugin for AIPlugin<T> {
    fn build(&self, app: &mut App) {
        self.setup_ai_stages(app);

        app.add_plugin(BigBrainPlugin)
            // Scoring Systems
            .add_system_set_to_stage(
                BigBrainStage::Scorers,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .with_system(can_see_player)
                    .into(),
            )
            // Action Systems
            .add_system_set_to_stage(
                AtrlStage::AIThinking,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .with_system(wander_action)
                    .with_system(chase_action)
                    .with_system(attack_action)
                    .into(),
            );
    }
}
