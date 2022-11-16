use crate::prelude::*;

static CONSUME_EVENTS_STAGE: &str = "consume_events";

pub struct EcsPlugin<T, R> {
    pub state_running: T,
    pub turn_state_ticking: R,
}

impl<T: StateNext, R: StateNext + Resource> Plugin for EcsPlugin<T, R> {
    fn build(&self, app: &mut App) {
        app.add_plugin(AIPlugin {
            state_running: self.state_running.clone(),
            turn_state_ticking: self.turn_state_ticking.clone(),
        });

        app.add_stage_before(CoreStage::PostUpdate, CONSUME_EVENTS_STAGE, SystemStage::parallel())
            .add_system_set_to_stage(
                CONSUME_EVENTS_STAGE,
                ConditionSet::new()
                    .run_in_state(self.state_running.clone())
                    .run_if_resource_equals(self.turn_state_ticking.clone())
                    .with_system(move_actors)
                    .into(),
            );
    }
}
