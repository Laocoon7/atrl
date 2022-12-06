use crate::prelude::*;
pub struct PlayerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<ActionQueue>()
            .add_system_set(
                ConditionSet::new().run_in_state(self.state_running).with_system(player_input).into(),
            );
    }
}
