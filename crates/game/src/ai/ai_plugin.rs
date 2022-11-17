use crate::prelude::*;

pub struct AIPlugin<T, R> {
    pub state_running: T,
    pub turn_state_ticking: R,
}

impl<T: StateNext, R: StateNext + Resource> Plugin for AIPlugin<T, R> {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            // Scoring Systems
            .add_system_set_to_stage(
                BigBrainStage::Scorers,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .run_if_resource_equals(self.turn_state_ticking.clone())
                    .with_system(can_see_player)
                    .into(),
            )
            // Action Systems
            .add_system_set_to_stage(
                BigBrainStage::Actions,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .run_if_resource_equals(self.turn_state_ticking.clone())
                    .with_system(wander_action)
                    .into(),
            );
    }
}