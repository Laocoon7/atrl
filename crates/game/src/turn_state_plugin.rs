use crate::prelude::*;

static CONSUME_EVENTS_STAGE: &str = "consume_events";

pub struct TurnStatePlugin<T, R> {
    pub state_running: T,
    pub turn_state_ticking: R,
}

impl<T: StateNext, R: StateNext + Resource> Plugin for TurnStatePlugin<T, R> {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::PostUpdate, CONSUME_EVENTS_STAGE, SystemStage::parallel());

        app.add_system_set_to_stage(
            CONSUME_EVENTS_STAGE,
            ConditionSet::new()
                .run_in_state(self.state_running.clone())
                .run_if_resource_equals(self.turn_state_ticking.clone())
                .with_system(move_actors)
                .into(),
        );
    }
}
