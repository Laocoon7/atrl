use crate::prelude::*;
pub struct AIPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for AIPlugin<T> {
    fn build(&self, app: &mut App) {
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
                BigBrainStage::Actions,
                ConditionSet::new()
                    .run_in_state(self.state_running)
                    .with_system(wander_action)
                    .with_system(chase_action)
                    .into(),
            );
    }
}
