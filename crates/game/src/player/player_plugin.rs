use crate::prelude::*;
pub struct PlayerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default()).add_system_set(
            ConditionSet::new().run_in_state(self.state_running).with_system(player_input).into(),
        );

        // TODO: Remove this once states are working for player / AI
        // app.add_system(insert_resource!(TurnState::AwaitingInput).
        // run_if_resource_equals(TurnState::Ticking));
    }
}
